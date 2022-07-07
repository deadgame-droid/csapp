use std::mem;

pub fn show_bytes(start: &u8, len: usize) {
    for i in 0..len {
        let res = unsafe { *(start as *const u8 as *mut u8).add(i) };
        print!(" {:0>2x}", res);
    }
    println!();
}
pub fn show_bytes_unchecked<T>(start: &T) {
    let byte_pointer: &u8;
    let len = mem::size_of::<T>();
    unsafe {
        byte_pointer = mem::transmute::<&T, &u8>(start);
    }
    show_bytes(byte_pointer, len);
}

pub fn show_int(x: &i32) {
    let len = mem::size_of::<i32>();
    let start: &u8;
    unsafe {
        start = mem::transmute::<&i32, &u8>(x);
    }
    show_bytes(start, len);
}

pub fn show_float(x: &f32) {
    let len = mem::size_of::<f32>();
    let start: &u8;
    unsafe {
        start = mem::transmute::<&f32, &u8>(x);
    }
    show_bytes(start, len);
}

pub fn show_pointer<T>(x: &T) {
    let len = mem::size_of::<T>();
    let start: &u8 = unsafe { mem::transmute::<&T, &u8>(x) };
    show_bytes(start, len);
}
