mod login;
mod packet_header;
mod prelogin;

pub use login::Login;
pub use packet_header::{PacketHeader, PacketType, Status};
pub use prelogin::{Encryption, Prelogin, PreloginOption, Version};

pub trait Encode {
    fn encode(&self, buf: &mut Vec<u8>);
}
