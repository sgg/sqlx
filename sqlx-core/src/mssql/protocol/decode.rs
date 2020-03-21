pub trait Decode<'de> {
    fn decode(buf: &'de [u8]) -> crate::Result<Self>
    where
        Self: Sized;
}
