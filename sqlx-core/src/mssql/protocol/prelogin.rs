use super::super::MsSql;
use super::Encode;
use super::PacketHeader;
use super::PacketType;
use super::Status;
use crate::io::Buf;
use crate::io::BufMut;
use byteorder::BigEndian;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use std::io;

const TERMINATOR: u8 = 0xFF;

// TODO: SSL_PAYLOAD
#[derive(Default, Debug)]
pub struct Prelogin<'de> {
    // Version is a `PreLoginOption`, but since it's required it's moved out of the `Prelogion.options`
    pub version: Version,
    // Encryption is a `PreLoginOption`, but since it's required it's moved out of the `Prelogion.options`
    pub encryption: Encryption,
    pub options: Vec<PreloginOption<'de>>,
}

// This isn't specified in the encoding part, but the way PreloginOptions are encoded is by first
// encoding the PreloginOptionToken PreloginOptionDataOffset PreloginOptionDataLength of each option first.
// Then, encoding the data part of each token afterwards.
#[derive(Debug)]
pub enum PreloginOption<'de> {
    InStopT(&'de [u8]),
    ThreadId(u32),
    Mars(u8),
    TraceId {
        conn_id: u128,
        activity: u128,
        seq_id: u32,
    },
    FedAuthRequired(u8),
    NonceOpt([u8; 32]),
}

#[derive(Default, Debug)]
pub struct Version {
    pub version: u32,
    pub build: u16,
}

#[derive(Copy, Clone, Debug)]
pub enum Encryption {
    Off = 0x00,
    On = 0x01,
    NotSupported = 0x02,
    Required = 0x03,
    ClientCertEncryptionOff = 0x80,
    ClientCertEncryptionOn = 0x81,
    ClientCertEncryptionReq = 0x83,
}

impl Default for Encryption {
    fn default() -> Encryption {
        Encryption::Off
    }
}

impl Encryption {
    pub(crate) fn read(buf: &[u8]) -> crate::Result<MsSql, Self> {
        use Encryption::*;

        match buf[0] {
            0x00 => Ok(Off),
            0x01 => Ok(On),
            0x02 => Ok(NotSupported),
            0x03 => Ok(Required),
            0x80 => Ok(ClientCertEncryptionOff),
            0x81 => Ok(ClientCertEncryptionOn),
            0x83 => Ok(ClientCertEncryptionReq),
            v => Err(protocol_err!("Received unsupported encryption value: {:?}", v).into()),
        }
    }
}

impl<'de> Encode for Prelogin<'de> {
    fn encode(&self, buf: &mut Vec<u8>) {
        // The way PacketHeader is handled here is we encode the entire packet header right away.
        // Then after encoding the packet itself we take the entire length of what we wrote,
        // including the packet header itself. At the end we simply index into the buffer and write
        // over the length. To put it simply, we reserve space for the header and then overwrite the
        // length after the packet has been written.
        let start = buf.len();
        let mut header = PacketHeader::new(PacketType::PreLogin);
        header.status = Status::END_OF_MESSAGE;

        header.encode(buf);

        // The offset begins after all the tokens are encoded
        // We can easily compute this by getting the length of options
        // + 1 because the version token is not within the options vector.
        // Then we add a single byte for the TERMINATOR token
        let mut offset = (self.options.len() + 1) * 5 + 1;
        dbg!(offset);

        // Version Token *MUST* be the first token so it's not part of the options vector. So, encode it first
        buf.push(0x00);
        buf.put_u16::<BigEndian>(offset as u16);
        buf.put_u16::<BigEndian>(6);

        offset += 6;

        // Encryption Token *MUST* be provided.
        buf.push(0x01);
        buf.put_u16::<BigEndian>(offset as u16);
        buf.put_u16::<BigEndian>(1);

        offset += 1;

        for opt in self.options.iter() {
            // pl_option_token
            buf.push(match opt {
                PreloginOption::InStopT(_) => 0x02,
                PreloginOption::ThreadId(_) => 0x03,
                PreloginOption::Mars(_) => 0x04,
                PreloginOption::TraceId { .. } => 0x05,
                PreloginOption::FedAuthRequired(_) => 0x06,
                PreloginOption::NonceOpt(_) => 0x07,
            });

            // pl_offset
            buf.put_u16::<BigEndian>(offset as u16);

            // this is the length of the data bit for an option.
            // we need the length because it's required in the token header,
            // and because we need to update offest accordingly for the next token.
            let len: u16 = match opt {
                PreloginOption::InStopT(vec) => vec.len() as u16 + 1,
                PreloginOption::ThreadId(_) => 4,
                PreloginOption::Mars(_) => 1,
                PreloginOption::TraceId { .. } => 36,
                PreloginOption::FedAuthRequired(_) => 1,
                PreloginOption::NonceOpt(_) => 32,
            };

            // pl_option_length
            buf.put_u16::<BigEndian>(len);

            offset += len as usize;
        }

        // Terminator is technically a `PreloginOption` as well, but it's special because:
        // 1) It is required to be the last option.
        // 2) It does not have an `offest` nor `length` which all the other options must have.
        buf.push(TERMINATOR);

        // Version encoded as the first option because that is required.
        // This is the data for the Version token
        buf.put_u32::<BigEndian>(self.version.version);
        buf.put_u16::<BigEndian>(self.version.build);

        // Encryption
        buf.push(self.encryption as u8);

        // Encode the DATA of the each option one after another
        for opt in self.options.iter() {
            match opt {
                PreloginOption::InStopT(vec) => {
                    buf.extend_from_slice(&vec);
                    // IntStopT vec *must* be followed by a 0x00 byte which is included in the offset
                    buf.push(0);
                }
                PreloginOption::ThreadId(id) => buf.put_u32::<LittleEndian>(*id),
                PreloginOption::Mars(mars) => buf.push(*mars),
                PreloginOption::TraceId {
                    conn_id,
                    activity,
                    seq_id,
                } => {
                    buf.extend_from_slice(&conn_id.to_be_bytes()[..]);
                    buf.extend_from_slice(&activity.to_be_bytes()[..]);
                    buf.put_u32::<LittleEndian>(*seq_id);
                }
                PreloginOption::FedAuthRequired(auth) => buf.push(*auth),
                PreloginOption::NonceOpt(nonce) => buf.extend_from_slice(&nonce[..]),
            }
        }

        // Not that we've encoded the entire packet including header, go back and update the length
        // in the buffer. This should be 2 bytes from the beginning of the buffer.
        let length = ((buf.len() - start) as u16).to_be_bytes();

        println!("{:X?}", &buf);
        buf[start + 2..start + 4].copy_from_slice(&length);
    }
}

// We need to be able to decode a PreLogin packet because it is what the server response with
// to a PreLogin request. However, the packet type is not 0x12, but 0x4 in the response.
impl<'de> Prelogin<'de> {
    pub(crate) fn read(mut buf: &'de [u8]) -> crate::Result<MsSql, Self> {
        let mut version = None;
        let mut encryption = None;
        let mut options: Vec<PreloginOption> = Vec::new();

        // Step by 5 because each token 5 bytes long
        for i in (0usize..).step_by(5) {
            // The first is the token, then offset, then length
            // This is only untrue for the token `TERMINATOR` since it doesn't have an offset nor length.
            // However since the token VERSION is required and is at least 4 bytes we can still call
            // BigEndian::read_u16 because the data does exist, it's just not valid for terminator.
            match (
                buf[i],
                BigEndian::read_u16(&buf[i + 1..]) as usize,
                BigEndian::read_u16(&buf[i + 3..]) as usize,
            ) {
                (0x00, offset, length) => {
                    version = Some(Version {
                        version: BigEndian::read_u32(&buf[offset..]),
                        build: BigEndian::read_u16(&buf[offset + 4..]),
                    });
                }
                (0x01, offset, length) => {
                    encryption = Some(Encryption::read(&buf[offset..])?);
                }
                (0x02, offset, length) => {
                    options.push(PreloginOption::InStopT(&buf[offset..offset + length]));
                }
                (0x03, offset, length) => options.push(PreloginOption::ThreadId(
                    BigEndian::read_u32(&buf[offset..]),
                )),
                (0x04, offset, length) => options.push(PreloginOption::Mars(buf[offset])),
                (0x05, offset, length) => {
                    options.push(PreloginOption::TraceId {
                        conn_id: BigEndian::read_u128(&buf[offset..]),
                        activity: BigEndian::read_u128(&buf[offset + 16..]),
                        seq_id: BigEndian::read_u32(&buf[offset + 32..]),
                    });
                }
                (0x06, offset, length) => {
                    options.push(PreloginOption::FedAuthRequired(buf.get_u8()?))
                }
                (0x07, offset, length) => {
                    let mut nonce = [0u8; 32];
                    nonce.copy_from_slice(&buf[offset..offset + 32]);
                    options.push(PreloginOption::NonceOpt(nonce));
                }
                (TERMINATOR, _, _) => break,
                v => return Err(protocol_err!("Received unprocessable token type {:?}", v).into()),
            }
        }

        // Version is *REQUIRED* to be set
        let version =
            version.ok_or(protocol_err!("Didn't receive version when one is expected"))?;

        // Encryption is *REQUIRED* to be set
        let encryption = encryption.ok_or(protocol_err!(
            "Didn't receive encryption when one is expected"
        ))?;

        Ok(Self {
            version,
            encryption,
            options,
        })
    }
}
