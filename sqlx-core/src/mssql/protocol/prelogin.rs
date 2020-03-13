pub struct PacketHeader {
    r#type: u8,
    status: u8,
    length: u16,
    spid: u16,
    packet: u8,
    window: u8,
}

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
}

pub struct InstValidity {
    data: Vec<u8>,
    // ALWAYS 0x00
    terminator: u8,
}
