use std::mem;

pub fn get_bytes_slice<T: Sized>(data: &[T]) -> &[u8] {
    let size = mem::size_of::<T>();
    unsafe {
        let ptr = data.as_ptr() as *const u8;
        let slice = std::slice::from_raw_parts(ptr, size * data.len());
        slice
    }
}

pub fn get_bytes<T: Sized>(data: &T) -> &[u8] {
    let size = mem::size_of::<T>();
    unsafe {
        let ptr = data as *const T as *const u8;
        let slice = std::slice::from_raw_parts(ptr, size);
        slice
    }
}
