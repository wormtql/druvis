use crate::utils;

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl<T> AsBytes for T where T: Sized {
    fn as_bytes(&self) -> &[u8] {
        utils::get_bytes(self)
    }
}
