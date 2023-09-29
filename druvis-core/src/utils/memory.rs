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

pub fn create_buffer(size: usize) -> Vec<u8> {
    let mut temp: Vec<u8> = Vec::with_capacity(size as usize);
    temp.resize(size, 0);
    temp
}

pub fn write_buffer(buf: &mut [u8], offset: usize, data: &[u8]) {
    let data_size = data.len();
    let buf_range = &mut buf[offset..offset + data_size];
    buf_range.copy_from_slice(data);
}

pub fn reinterpret_slice<T, U>(buf: &[T]) -> &[U] {
    let size1 = mem::size_of::<T>();
    let size2 = mem::size_of::<U>();

    if buf.len() * size1 % size2 != 0 {
        panic!("cannot reinterpret");
    }
    let elements = buf.len() * size1 / size2;

    unsafe {
        let ptr = buf.as_ptr() as *const U;
        std::slice::from_raw_parts(ptr, elements)
    }
}
