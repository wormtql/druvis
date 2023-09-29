use crate::utils;

pub trait AsBytes {
    fn druvis_as_bytes(&self) -> &[u8];
}

impl<T> AsBytes for T where T: Sized {
    fn druvis_as_bytes(&self) -> &[u8] {
        utils::get_bytes(self)
    }
}
