use std::mem;

pub fn show_bytes(start: *const u8, len: usize) {
    for i in 0..len {
        let res;
        unsafe {
            res = *start.add(i);
        }
        print!(" {:0>2x}", res);
    }
    println!();
}
pub fn show_bytes_unchecked<T>(start: &T) {
    let byte_pointer: *const u8;
    let len = mem::size_of::<T>();
    unsafe { byte_pointer = mem::transmute::<&T,*const u8>(start); }
    show_bytes(byte_pointer, len);
}

pub fn show_int(x: &i32) {
    let len = mem::size_of::<i32>();
    let start: *const u8;
    unsafe {
        start = mem::transmute::<&i32, *const u8>(x);
    }
    show_bytes(start, len);
}

pub fn show_float(x: &f32) {
    let len = mem::size_of::<f32>();
    let start: *const u8;
    unsafe {
        start = mem::transmute::<&f32, *const u8>(x);
    }
    show_bytes(start, len);
}

pub fn show_pointer<T>(x: *const T) {
    let len = mem::size_of::<T>();
    let start: *const u8 = unsafe { mem::transmute::<*const T, *const u8>(x) };
    show_bytes(start, len);
}
