use super::super::MsSql;
use super::Decode;
use super::Encode;
use super::PacketHeader;
use super::PacketType;
use crate::io::Buf;
use crate::io::BufMut;
use crate::Result;
use byteorder::BigEndian;
use byteorder::ByteOrder;
use byteorder::LittleEndian;

static TERMINATOR: u8 = 0xFF;

// TODO: SSL_PAYLOAD
pub struct Prelogin {
    // Version is a `PreLoginOption`, but since it's required it's moved out of the `Prelogion.options`
    version: Version,
    options: Vec<PreloginOption>,
}

// This isn't specified in the encoding part, but the way PreloginOptions are encoded is by first
// encoding the PreloginOptionToken PreloginOptionDataOffset PreloginOptionDataLength of each option first.
// Then, encoding the data part of each token afterwards.
pub enum PreloginOption {
    Encryption(u8),
    InStopT(Vec<u8>),
    ThreadId(u32),
    Mars(u8),
    TraceId {
        conn_id: [u8; 16],
        activity: [u8; 16],
        seq_id: u32,
    },
    FedAuthRequired(u8),
    NonceOpt([u8; 32]),
}

pub struct Version {
    major: u8,
    minor: u8,
    build: u16,
}

impl Encode for Prelogin {
    fn encode(&self, buf: &mut Vec<u8>) {
        // The way PacketHeader is handled here is we encode the entire packet header right away.
        // Then after encoding the packet itself we take the entire length of what we wrote,
        // including the packet header itself. At the end we simply index into the buffer and write
        // over the length. To put it simply, we reserve space for the header and then overwrite the
        // length after the packet has been written.
        let start = buf.len();

        let header = PacketHeader::new(PacketType::PreLogin);

        header.encode(buf);

        // The offset begins after all the tokens are encoded
        // We can easily compute this by getting the length of options
        // + 1 because the version token is not within the options vector.
        // Then we add a single byte for the TERMINATOR token
        let mut offset = (self.options.len() + 1) * 5 + 1;

        // Version Token *MUST* be the first token so it's not part of the options vector.
        // So, encode it first
        buf.push(0x00);
        // The offset for version should be right after we encode all the token "headers"/"pointers"
        buf.put_u16::<BigEndian>(offset as u16);
        // The length of the data for a Version token is 4 bytes
        buf.put_u16::<BigEndian>(4);

        offset += 4;

        for opt in self.options.iter() {
            use PreloginOption::*;

            // PL_OPTION_TOKEN
            buf.push(match opt {
                Encryption(_) => 0x01,
                InStopT(_) => 0x02,
                ThreadId(_) => 0x03,
                Mars(_) => 0x04,
                TraceId { .. } => 0x05,
                FedAuthRequired(_) => 0x06,
                NonceOpt(_) => 0x07,
            });

            // PL_OFFSET
            buf.put_u16::<BigEndian>(offset as u16);

            // This is the length of the DATA bit for an option.
            // We need the length because it's required in the token header,
            // and because we need to update offest accordingly for the next token.
            let len: u16 = match opt {
                Encryption(_) => 1,
                InStopT(vec) => vec.len() as u16 + 1,
                ThreadId(_) => 4,
                Mars(_) => 1,
                TraceId { .. } => 36,
                FedAuthRequired(_) => 1,
                NonceOpt(_) => 32,
            };

            // PL_OPTION_LENGTH
            buf.put_u16::<BigEndian>(len);

            offset += len as usize;
        }

        // Terminator is technically a `PreloginOption` as well, but it's special because:
        // 1) It is required to be the last option.
        // 2) It does not have an `offest` nor `length` which all the other options must have.
        buf.push(TERMINATOR);

        // Version encoded as the first option because that is required.
        // This is the data for the Version token
        buf.push(self.version.major);
        buf.push(self.version.minor);
        buf.put_u16::<BigEndian>(self.version.build);

        // Encode the DATA of the each option one after another
        for opt in self.options.iter() {
            use PreloginOption::*;

            match opt {
                Encryption(enc) => buf.push(*enc),
                InStopT(vec) => {
                    buf.extend_from_slice(&vec);
                    // IntStopT vec *must* be followed by a 0x00 byte which is included in the offset
                    buf.push(0);
                }
                ThreadId(id) => buf.put_u32::<LittleEndian>(*id),
                Mars(mars) => buf.push(*mars),
                TraceId {
                    conn_id,
                    activity,
                    seq_id,
                } => {
                    buf.extend_from_slice(&conn_id[..]);
                    buf.extend_from_slice(&activity[..]);
                    buf.put_u32::<LittleEndian>(*seq_id);
                }
                FedAuthRequired(auth) => buf.push(*auth),
                NonceOpt(nonce) => buf.extend_from_slice(&nonce[..]),
            }
        }

        // Not that we've encoded the entire packet including header, go back and update the length
        // in the buffer. This should be 2 bytes from the beginning of the buffer.
        let length = ((buf.len() - start) as u16).to_be_bytes();
        buf[start + 2..start + 4].copy_from_slice(&length);
    }
}

// We need to be able to decode a PreLogin packet because it is what the server response with
// to a PreLogin request. However, the packet type is not 0x12, but 0x4 in the response.
impl Decode for Prelogin {
    fn decode(mut buf: &[u8]) -> Result<Self> {
        let header = PacketHeader::decode(&buf)?;

        let mut version = None;
        let mut options: Vec<PreloginOption> = Vec::new();
        loop {
            match buf.get_u8()? {
                0x00 => {
                    let offset = buf.get_u16::<BigEndian>()?;
                    let _length = buf.get_u16::<BigEndian>()?;

                    version = Some(Version {
                        major: buf[offset as usize],
                        minor: buf[(offset + 1) as usize],
                        build: BigEndian::read_u16(&buf[(offset + 2) as usize..]),
                    });
                }
                0x01 => todo!(),
                0x02 => todo!(),
                0x03 => todo!(),
                0x04 => todo!(),
                0x05 => todo!(),
                0x06 => todo!(),
                0x07 => todo!(),
                v => return Err(protocol_err!("Received unprocessable token type {:?}", v).into()),
            }
        }

        let version =
            version.ok_or(protocol_err!("Didn't receive version when one is expected"))?;

        Ok(Self { version, options })
    }
}
