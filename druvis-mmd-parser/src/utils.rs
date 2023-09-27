use std::mem;

pub fn read<T: Sized>(data: &[u8], position: &mut usize) -> T {
    let size = mem::size_of::<T>();
    let result = unsafe {
        let slice = &data[*position..*position + size];
        let ptr = slice.as_ptr() as *const T;
        mem::transmute_copy(&*ptr)
    };
    *position += size;
    result
}

pub fn read_var<T: Sized>(data: &[u8], position: &mut usize, size: usize) -> Vec<T> {
    let element_size = mem::size_of::<T>();
    let mut result = Vec::new();
    for _ in 0..size {
        result.push(read::<T>(data, position));
    }
    result
}

pub fn read_text(data: &[u8], position: &mut usize) -> (i32, Vec<u8>) {
    let length = read::<i32>(data, position);
    let text = read_var::<u8>(data, position, length as usize);
    (length, text)
}