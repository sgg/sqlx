use super::Encode;
use crate::io::BufMut;
use byteorder::BigEndian;
use byteorder::LittleEndian;

pub struct PacketHeader {
    r#type: u8,
    status: u8,
    length: u16,
    spid: u16,
    packet: u8,
    window: u8,
}

static TERMINATOR: u8 = 0xFF;

// TODO: SSL_PAYLOAD
pub struct Prelogin {
    // Version is a `PreLoginOption`, but since it's required it's moved out of the `Prelogion.options`
    version: Version,
    options: Vec<PreloginOption>,
}

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
// TODO: SSL_PAYLOAD
pub struct Prelogin {
    options: Vec<(PreloginOption, Vec<u8>)>,
}

pub enum PreloginOption {
    Some {
        token: u8,
        // BigEndian
        offset: u16,
        // BigEndian
        len: u16,
    },
    None {
        // ALWAYS 0xFF
        terminator: u8,
    },
}

pub struct Version {
    major: u8,
    minor: u8,
    build: u16,
}

// TODO: Include packet header with type
// Prelogin Token Type: 0x12
impl Encode for Prelogin {
    fn encode(&self, buf: &mut Vec<u8>) {
        let mut offset = 0u16;

        // Version Token and length both of whicha are constant
        buf.push(0x00);
        buf.push(4);

        // Can be moved into initial offset value, but it's more
        // clear what is happening by adding the length of Version
        // here instead. Plus the compiler will simply perform
        // constant propagation and clean this up, _hopefully_.
        offset += 4;

        for opt in self.options.iter() {
            use PreloginOption::*;

            buf.push(match opt {
                Encryption(_) => 0x01,
                InStopT(_) => 0x02,
                ThreadId(_) => 0x03,
                Mars(_) => 0x04,
                TraceId { .. } => 0x05,
                FedAuthRequired(_) => 0x06,
                NonceOpt(_) => 0x07,
            });

            buf.put_u16::<LittleEndian>(offset);

            let len: u16 = match opt {
                Encryption(_) => 1,
                InStopT(vec) => vec.len() as u16 + 1,
                ThreadId(_) => 4,
                Mars(_) => 1,
                TraceId { .. } => 36,
                FedAuthRequired(_) => 1,
                NonceOpt(_) => 32,
            };

            offset += len;

            buf.put_u16::<BigEndian>(len);
        }

        // Terminator is technically a `PreloginOption` as well, but it's special because:
        // 1) It is required to be the last option.
        // 2) It does not have an `offest` nor `length` which all the other options must have.
        buf.push(TERMINATOR);

        // Version encoded as the first option because that is required.
        buf.push(self.version.major);
        buf.push(self.version.minor);
        buf.put_u16::<BigEndian>(self.version.build);

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
    }
}

pub struct InstValidity {
    data: Vec<u8>,
    // ALWAYS 0x00
    terminator: u8,
}
