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
#[test]
fn ex_2_17() {
    const W: u32 = 4;
    // 2.1
    fn b2u(src: &str) -> u8 {
        let mut sum = 0u32;
        for (idx, c) in src.char_indices() {
            match c {
                '1' => sum += 2u32.pow(W - 1 - idx as u32),
                '0' => {}
                _ => panic!("{} is not 0 or 1", c),
            }
        }
        sum as u8
    }
    // 2.3
    fn b2t(src: &str) -> i8 {
        let mut sum = 0i32;
        let mut char_indices = src.char_indices();
        let (idx, c) = char_indices.next().unwrap();
        if c == '1' {
            sum = -2i32.pow(W - 1 - idx as u32);
        }
        for (idx, c) in char_indices {
            match c {
                '1' => sum += 2i32.pow(W - 1 - idx as u32),
                '0' => {}
                _ => panic!("{} is not 0 or 1", c),
            }
        }
        sum as i8
    }
    // 一位数的16进制数
    fn x2b(src: &str) -> String {
        let mut bin = String::new();
        let src = if src.starts_with("0x") || src.starts_with("0X") {
            src.get(2..).unwrap()
        } else {
            src
        };
        let mut d = u32::from_str_radix(src, 16).unwrap();
        for i in (0..=3).rev() {
            let t = 2u32.pow(i as u32);
            if d / t == 1 {
                bin.push('1');
                d -= t;
            } else {
                bin.push('0');
            }
        }
        bin
    }
    let test_data = ["0xe", "0x0", "0x5", "0x8", "0xD", "0xF"];
    println!("\n\t十六进制\t二进制\tB2U\tB2T");
    for x in test_data {
        let b = &(x2b(x));
        println!("\t{}\t\t{}\t{}\t{}", x, b, b2u(b), b2t(b));
    }
}

#[test]
fn ex_2_23() {
    fn fun1(word: u32) -> i32 {
        ((word << 24) >> 24) as i32
    }
    fn fun2(word: u32) -> i32 {
        ((word as i32) << 24) >> 24
    }
    let ws: [u32; 4] = [0x00000076, 0x87654321, 0x000000C9, 0xEDCBA987];
    println!("\n\tw\t\tfun1(w)\t\tfun2(w)");
    for w in ws {
        println!("\t0x{:0>8X}\t0x{:0>8X}\t0x{:0>8X}", w, fun1(w), fun2(w));
    }
}
