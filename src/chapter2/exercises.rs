use super::*;
#[test]
fn ex_2_7() {
    let s = "abcdef";
    let sp = s.as_ptr();
    show_bytes(sp, s.len());
}
#[test]
fn ex_2_9() {
    use super::Color::*;

    let blue = Blue;
    let green = Green;
    let blue_green = BlueGreen;
    let yellow = Yellow;
    let red = Red;
    let red_purple = RedPurple;
    println!();
    println!("Blue | Green = {:?}", blue | green);
    println!("Yellow & BlueGreen = {:?}", yellow & blue_green);
    println!("Red ^ RedPurple = {:?}", red ^ red_purple);
}
#[test]
fn ex_2_10() {
    let mut x = 123;
    let mut y = 456;
    println!("\nx = {}, y = {}", x, y);
    unsafe {
        inplace_swap(&mut x as *mut i32, &mut y as *mut i32);
    };
    println!("x = {}, y = {}", x, y);
}

#[test]
fn ex_2_11() {
    let mut a = [1, 2, 3, 4, 5];
    reverse_array(a.as_mut_ptr(), a.len());
    println!("{:?}", a);
}

// 除了x的最低有效位字节外，其他位置均置为0
fn keep_lowest_byte_and_set_others_zero(x: &mut u32) {
    *x &= 0xff;
}

// 除了x的最低有效位字节外，其他的位都取补，最低有效位字节保持不变。
fn keep_lowest_byte_and_set_others_complement(x: &mut u32) {
    // 取反等于异或1
    *x = !*x ^ 0xff;
}

// x的最低有效字节设置成全1，其他字节都保持不变。
fn set_lowest_byte_one(x: &mut u32) {
    *x |= 0xff;
}

#[test]
fn ex_2_12() {
    let mut x = 0x87654321u32;
    keep_lowest_byte_and_set_others_zero(&mut x);
    println!("\n{:0>8X}", x);

    let mut x = 0x87654321u32;
    keep_lowest_byte_and_set_others_complement(&mut x);
    println!("{:0>8X}", x);

    let mut x = 0x87654321u32;
    set_lowest_byte_one(&mut x);
    println!("{:0>8X}", x);
}
