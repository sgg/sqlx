use super::super::MsSql;
use super::Decode;
use super::Encode;
use crate::io::Buf;
use crate::io::BufMut;
use crate::Result;
use bitflags::bitflags;
use byteorder::BigEndian;

pub struct PacketHeader {
    // Type defines the type of message. Typeis a 1-byte unsigned char. The following table
    // describes the types that are available.
    //
    // If an unknown Typeis specified, the message receiver SHOULD disconnect the connection.
    // If a valid Typeis specified, but is unexpected (per section 3), the message receiver
    // SHOULD disconnect the connection. This applies to both the client and the server.
    // For example, the server could disconnect the connection if the server receives a message
    // with Typeequal 16 when the connection is already logged in.
    pub r#type: PacketType,

    // Status is a bit field used to indicate the message state. Statusis a 1-byte unsigned char.
    pub status: Status,

    // Length is the size of the packet including the 8 bytes in the packet header. It is
    // the number of bytes from the start of this header to the start of the next
    // packet header. Length is a 2-byte, unsigned short int and is represented in network
    // byte order (big-endian).
    //
    // The Lengthvalue MUST be greater than or equal to 512 bytes and smaller than or equal
    // to 32,767 bytes. The default value is 4,096 bytes.
    //
    // Starting with TDS 7.3, the Length MUST be the negotiated packet size when sending a packet
    // from client to server, unless it is the last packet of a request (that is, the EOM bit in
    // Status is ON) or the client has not logged in.
    pub length: u16,

    // Spid is the process ID on the server, corresponding to the current connection. This
    // information is sent by the server to the client and is useful for identifying
    // which thread on the server sent the TDS packet. It is provided for debugging
    // purposes. The client MAY send the SPID value to the server. If the client does not,
    // then a value of 0x0000 SHOULD be sent to the server. This is a 2-byte value and
    // is represented in network byte order (big-endian).
    pub spid: u16,

    // PacketID is used for numbering message packets that contain data in addition to the packet
    // header. PacketID is a 1-byte, unsigned char. Each time packet data is sent, the value of
    // PacketIDis incremented by 1,modulo 256. This allows the receiver to track the sequence
    // of TDS packets for a given message. This value is currently ignored.
    pub packet: u8,

    // This 1 byte is currently not used. This byte SHOULD be set to 0x00 and SHOULD be
    // ignored by the receiver.
    pub window: u8,
}

impl PacketHeader {
    pub fn new(r#type: PacketType) -> Self {
        Self {
            r#type,
            status: Status::NORMAL,
            length: 0,
            packet: 1,
            spid: 0,
            window: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub enum PacketType {
    SqlBatch = 1,
    PreTds7Login = 2,
    Rpc = 3,
    TabularResult = 4,
    AttentionSignal = 6,
    BulkLoadData = 7,
    FederatedAuthToken = 8,
    TransactionManagerRequest = 14,
    Tds7Login = 16,
    Sspi = 17,
    PreLogin = 18,
}

impl<'de> Decode<'de> for PacketType {
    fn decode(mut buf: &'de [u8]) -> Result<Self> {
        match buf.get_u8()? {
            1 => Ok(PacketType::SqlBatch),
            2 => Ok(PacketType::PreTds7Login),
            3 => Ok(PacketType::Rpc),
            4 => Ok(PacketType::TabularResult),
            6 => Ok(PacketType::AttentionSignal),
            7 => Ok(PacketType::BulkLoadData),
            8 => Ok(PacketType::FederatedAuthToken),
            14 => Ok(PacketType::TransactionManagerRequest),
            16 => Ok(PacketType::Tds7Login),
            17 => Ok(PacketType::Sspi),
            18 => Ok(PacketType::PreLogin),
            v => Err(protocol_err!("Unrecognized PacketType {:?}", v).into()),
        }
    }
}

bitflags! {
    pub struct Status: u8 {
        // "Normal" message.
        const NORMAL = 0x00;

        // End of message (EOM). The packet is the last packet in the whole request.
        const END_OF_MESSAGE = 0x01;

        // (From client to server) Ignore this event (0x01 MUST also be set).
        const IGNORE_EVENT = 0x02;

        // RESETCONNECTION
        //
        // (Introduced in TDS 7.1)
        //
        // (From client to server) Reset this connection
        // before processing event. Only set for event types Batch, RPC, or Transaction Manager
        // request. If clients want to set this bit, it MUST be part of the first packet of the
        // message. This signals the server to clean up the environment state of the connection
        // back to the default environment setting, effectively simulating a logout and a
        // subsequent login, and provides server support for connection pooling. This bit SHOULD
        // be ignored if it is set in a packet that is not the first packet of the message.
        //
        // This status bit MUST NOT be set in conjunction with the RESETCONNECTIONSKIPTRAN bit.
        // Distributed transactions and isolation levels will not be reset.
        const RESET_CONN = 0x08;

        // RESETCONNECTIONSKIPTRAN
        //
        // (Introduced in TDS 7.3)
        //
        // (From client to server) Reset the
        // connection before processing event but do not modify the transaction state (the
        // state will remain the same before and after the reset). The transaction in the
        // session can be a local transaction that is started from the session or it can
        // be a distributed transaction in which the session is enlisted. This status bit
        // MUST NOT be set in conjunction with the RESETCONNECTION bit.
        // Otherwise identical to RESETCONNECTION.
        const RESET_CONN_SKIP_TRAN = 0x10;
    }
}

impl Encode for PacketHeader {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(self.r#type as u8);
        buf.push(self.status.bits());
        buf.put_u16::<BigEndian>(self.length);
        buf.put_u16::<BigEndian>(self.spid);
        buf.push(self.packet);
        buf.push(self.window);
    }
}

impl<'de> Decode<'de> for PacketHeader {
    fn decode(mut buf: &'de [u8]) -> Result<Self> {
        Ok(Self {
            r#type: PacketType::decode(&buf)?,
            status: Status::from_bits_truncate(buf.get_u8()?),
            length: buf.get_u16::<BigEndian>()?,
            spid: buf.get_u16::<BigEndian>()?,
            packet: buf.get_u8()?,
            window: buf.get_u8()?,
        })
    }
}
