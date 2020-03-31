use crate::io::{Buf, BufMut};
use crate::mssql::protocol::Encode;
use bitflags::bitflags;
use byteorder::BigEndian;
use std::borrow::Cow;
use uuid::Uuid;

const TERMINATOR: u8 = 0xFF;

/// A message sent by the client to set up context for login. The server responds to a client
/// `PRELOGIN` message with a message of packet header type `0x04` and the packet data
/// containing a `PRELOGIN` structure.
#[derive(Debug)]
pub struct PreLogin<'a> {
    pub version: Version,
    pub options: Cow<'a, [PreLoginOption<'a>]>,
}

impl<'de> PreLogin<'de> {
    fn decode(mut buf: &'de [u8]) -> crate::Result<Self> {
        let mut version = None;
        let mut options = Vec::<PreLoginOption<'de>>::new();
        let mut offsets = buf;

        loop {
            let token = offsets.get_u8()?;

            match PreLoginOptionToken::decode(token) {
                Some(token) => {
                    let offset = offsets.get_u16::<BigEndian>()? as usize;
                    let size = offsets.get_u16::<BigEndian>()? as usize;
                    let mut data = &buf[offset..offset + size];

                    match token {
                        PreLoginOptionToken::Version => {
                            let major = data.get_u8()?;
                            let minor = data.get_u8()?;
                            let build = data.get_u16::<BigEndian>()?;
                            let sub_build = data.get_u16::<BigEndian>()?;

                            version = Some(Version {
                                major,
                                minor,
                                build,
                                sub_build,
                            });
                        }

                        PreLoginOptionToken::Encryption => {
                            options.push(PreLoginOption::Encryption(Encrypt::from_bits_truncate(
                                data.get_u8()?,
                            )));
                        }

                        tok => todo!("{:?}", tok),
                    }
                }

                None if token == TERMINATOR => {
                    break;
                }

                None => {
                    return Err(protocol_err!(
                        "PRELOGIN: unexpected login option token: 0x{:02?}",
                        token
                    )
                    .into());
                }
            }
        }

        let version = version.ok_or(protocol_err!("PRELOGIN: missing required VERSION option"))?;

        Ok(Self {
            version,
            options: Cow::Owned(options),
        })
    }
}

impl Encode for PreLogin<'_> {
    fn encode(&self, buf: &mut Vec<u8>) {
        // NOTE: Packet headers are written in MsSqlStream::write

        // Rules
        //  PRELOGIN = (*PRELOGIN_OPTION *PL_OPTION_DATA) / SSL_PAYLOAD
        //  PRELOGIN_OPTION = (PL_OPTION_TOKEN PL_OFFSET PL_OPTION_LENGTH) / TERMINATOR

        // Offset must refer to the position after all options are encoded
        // This can easily be computed by getting the length of the options + 1 because the
        // version token is separate. Additionally there is one more byte for the terminator.
        let mut offset = ((self.options.len() + 1) * 5 + 1) as u16;

        // NOTE: VERSION is a required token, and it MUST be the
        //       first token sent as part of PRELOGIN.
        PreLoginOptionToken::Version.encode(buf, &mut offset, 6);

        for opt in &*self.options {
            opt.token().encode(buf, &mut offset, opt.size());
        }

        // TERMINATOR ends the sequence of PRELOGIN_OPTION
        buf.push(TERMINATOR);

        self.version.encode(buf);

        for opt in &*self.options {
            opt.encode(buf);
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum PreLoginOptionToken {
    Version = 0x00,
    Encryption = 0x01,
    Instance = 0x02,
    ThreadId = 0x03,
    MultipleActiveResultSets = 0x04, // MARS
    TraceId = 0x05,
}

impl PreLoginOptionToken {
    fn encode(self, buf: &mut Vec<u8>, offset: &mut u16, len: u16) {
        buf.push(self as u8);
        buf.put_u16::<BigEndian>(*offset);
        buf.put_u16::<BigEndian>(len);

        *offset += len;
    }

    fn decode(b: u8) -> Option<Self> {
        Some(match b {
            0x00 => PreLoginOptionToken::Version,
            0x01 => PreLoginOptionToken::Encryption,
            0x02 => PreLoginOptionToken::Instance,
            0x03 => PreLoginOptionToken::ThreadId,
            0x04 => PreLoginOptionToken::MultipleActiveResultSets,
            0x05 => PreLoginOptionToken::TraceId,

            _ => {
                return None;
            }
        })
    }
}

bitflags! {
    /// During the Pre-Login handshake, the client and the server negotiate the
    /// wire encryption to be used.
    pub struct Encrypt: u8 {
        /// Encryption is available but on.
        const ON = 0x01;

        /// Encryption is not available.
        const NOT_SUPPORTED = 0x02;

        /// Encryption is required.
        const REQUIRED = 0x03;

        /// The client certificate should be used to authenticate
        /// the user in place of a user/password.
        const CLIENT_CERT = 0x80;
    }
}

#[derive(Debug, Clone)]
pub enum PreLoginOption<'a> {
    Encryption(Encrypt),

    /// Name of the instance of the database server.
    Instance(&'a str),

    ThreadId(u32),

    TraceId {
        connection_id: Uuid,
        activity_id: Uuid,
        activity_seq: u32,
    },

    MultipleActiveResultSets(bool),
}

impl PreLoginOption<'_> {
    fn encode(&self, buf: &mut Vec<u8>) {
        match self {
            PreLoginOption::Encryption(encrypt) => {
                buf.push(encrypt.bits());
            }

            PreLoginOption::Instance(name) => {
                buf.extend_from_slice(name.as_bytes());
                buf.push(b'\0');
            }

            PreLoginOption::MultipleActiveResultSets(enabled) => {
                buf.push(*enabled as u8);
            }

            PreLoginOption::ThreadId(id) => {
                buf.put_u32::<BigEndian>(*id);
            }

            PreLoginOption::TraceId {
                connection_id,
                activity_id,
                activity_seq,
            } => {
                buf.extend_from_slice(connection_id.as_bytes());
                buf.extend_from_slice(activity_id.as_bytes());
                buf.put_u32::<BigEndian>(*activity_seq);
            }
        }
    }

    fn token(&self) -> PreLoginOptionToken {
        match self {
            PreLoginOption::Encryption(_) => PreLoginOptionToken::Encryption,
            PreLoginOption::Instance(_) => PreLoginOptionToken::Instance,
            PreLoginOption::ThreadId(_) => PreLoginOptionToken::ThreadId,
            PreLoginOption::TraceId { .. } => PreLoginOptionToken::TraceId,
            PreLoginOption::MultipleActiveResultSets(_) => {
                PreLoginOptionToken::MultipleActiveResultSets
            }
        }
    }

    fn size(&self) -> u16 {
        match self {
            PreLoginOption::Encryption(_) => 1,
            PreLoginOption::Instance(name) => name.len() as u16 + 1,
            PreLoginOption::ThreadId(_) => 4,
            PreLoginOption::TraceId { .. } => 36,
            PreLoginOption::MultipleActiveResultSets(_) => 1,
        }
    }
}

#[derive(Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub build: u16,
    pub sub_build: u16,
}

impl Version {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(self.major);
        buf.push(self.minor);
        buf.put_u16::<BigEndian>(self.build);
        buf.put_u16::<BigEndian>(self.sub_build);
    }
}

#[cfg(test)]
#[test]
fn test_encode_pre_login() {
    let mut buf = Vec::new();

    let pre_login = PreLogin {
        version: Version {
            major: 9,
            minor: 0,
            build: 0,
            sub_build: 0,
        },
        options: Cow::Borrowed(&[
            PreLoginOption::Encryption(Encrypt::ON),
            PreLoginOption::Instance(""),
            PreLoginOption::ThreadId(0xB80D0000),
            PreLoginOption::MultipleActiveResultSets(true),
        ]),
    };

    // From v20191101 of MS-TDS
    #[rustfmt::skip]
    let expected = vec![
        0x00, 0x00, 0x1A, 0x00, 0x06, 0x01, 0x00, 0x20, 0x00, 0x01, 0x02, 0x00, 0x21, 0x00, 
        0x01, 0x03, 0x00, 0x22, 0x00, 0x04, 0x04, 0x00, 0x26, 0x00, 0x01, 0xFF, 0x09, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0xB8, 0x0D, 0x00, 0x00, 0x01
    ];

    pre_login.encode(&mut buf);

    assert_eq!(expected, buf);
}

#[cfg(test)]
#[test]
fn test_decode_pre_login() {
    #[rustfmt::skip]
    let buffer = [
        0, 0, 11, 0, 6, 1, 0, 17, 0, 1, 255,
        14, 0, 12, 209, 0, 0, 0,
    ];

    let pre_login = PreLogin::decode(&buffer[..]).unwrap();

    // v14.0.3281
    assert_eq!(pre_login.version.major, 14);
    assert_eq!(pre_login.version.minor, 0);
    assert_eq!(pre_login.version.build, 3281);
    assert_eq!(pre_login.version.sub_build, 0);

    // ENCRYPT_OFF
    assert!(matches!(
        pre_login.options[0],
        PreLoginOption::Encryption(enc) if enc.is_empty()
    ));
}
