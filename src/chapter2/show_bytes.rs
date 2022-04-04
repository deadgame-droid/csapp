use std::mem;

type BytePointer = *const u8;

pub fn show_bytes(start: BytePointer, len: usize) {
    for i in 0..len {
        let res;
        unsafe {
            res = *start.add(i);
        }
        print!(" {:0>2x}", res);
    }
    println!();
}

pub fn show_int(x: i32) {
    let len = mem::size_of::<i32>();
    let start: BytePointer;
    unsafe {
        start = mem::transmute::<&i32, BytePointer>(&x);
    }
    show_bytes(start, len);
}

pub fn show_float(x: f32) {
    let len = mem::size_of::<f32>();
    let start: BytePointer;
    unsafe {
        start = mem::transmute::<&f32, BytePointer>(&x);
    }
    show_bytes(start, len);
}

pub fn show_pointer<T>(x: *const T) {
    let len = mem::size_of::<T>();
    let start: BytePointer = unsafe { mem::transmute::<*const T, BytePointer>(x) };
    show_bytes(start, len);
}
