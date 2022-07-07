use super::*;
#[test]
fn _7() {
    let s = "abcdef";
    let sp = s.as_bytes();
    show_bytes(&sp[0], s.len());
}
#[test]
fn _9() {
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
fn _10() {
    let mut x = 123;
    let mut y = 456;
    println!("\nx = {}, y = {}", x, y);
    unsafe {
        inplace_swap(&mut x as *mut i32, &mut y as *mut i32);
    };
    println!("x = {}, y = {}", x, y);
}

#[test]
fn _11() {
    let mut a = [1, 2, 3, 4, 5];
    reverse_array(&mut a[0], 5);
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
fn _12() {
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
fn _17() {
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
            sum = -(2i32.pow(W - 1 - idx as u32));
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
    // 一位16进制数
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
fn _23() {
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

#[test]
fn _25() {
    fn sum_elements(a: &[f32], length: usize) -> f32 {
        let mut result: f32 = 0.0;
        let mut i: usize = 0;
        while i < length {
            result += a[i];
            i += 1;
        }
        result
    }

    println!("\n{}", sum_elements(&[1.0, 2.0, 3.0, 4.0], 4));
    // 数组越界访问错误，直接panic
    // println!("\n{}", sum_elements(&[],0));
}
#[test]
fn _42() {
    fn div16(x: i32) -> i32 {
        // 负数向下舍入，比如：771.25 -> 772
        // 正数舍去小数
        // 负数的偏执值为15，正数为0
        let bias = (x >> 31) & 0xF;
        (x + bias) >> 4
    }
    assert_eq!(-17 / 16, div16(-17));
}

#[test]
fn _43() {
    const M: i32 = 2i32.pow(5) - 1;
    const N: i32 = 2i32.pow(3);

    fn arith(x: i32, y: i32) -> i32 {
        x * M + y / N
    }

    fn optarith(mut x: i32, mut y: i32) -> i32 {
        let t = x;
        x <<= 5;
        x -= t;

        if y < 0 {
            y += 7;
            y >>= 3;
        } else {
            y >>= 3;
        }

        x + y
    }

    assert_eq!(arith(4, -9), optarith(4, -9));
}

#[allow(unused)]
#[test]
fn _44() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    println!();
    x = i32::MIN;
    println!(
        "A. (x > 0) || (x - 1 < 0) => {}, x = {:#b}",
        (x > 0) || (x.wrapping_sub(1) < 0),
        x
    );
    println!(
        "B. (x & 7) != 7 || ((x << 29) < 0) => {}",
        (x & 7) != 7 || ((x << 29) < 0)
    );
    x = -65535; // 46341 <= x <= 65535 and -65535 <= x <= -46341
    println!("C. (x * x) >= 0 => {}", (x.wrapping_mul(x)) >= 0);
}

#[test]
fn _46() {
    // x = 0b0.00011001100110011001100
    // A. 0.1 - x = 0b0.00000000000000000000000[1100]...
    // B. 0.1 - x = 0.00000009;
    println!(
        "100 hours's deference: {}s",
        0.00000009 * 100.0 * 60.0 * 60.0 * 10.0
    );
}

// _47()
// see md/chapter2/_47.md

#[test]
fn _48() {
    // decimal: 3510593
    let x: i32 = 0x00359141;
    // 32位整数转为单精度，使用u32来表示。
    fn int2float(x: i32) -> u32 {
        let mut shf_bits: u32 = 0;
        let mut f = x as u32;
        while f & 0x80000000 == 0 {
            f <<= 1;
            shf_bits += 1;
        }
        f <<= 1;
        shf_bits += 1;
        let e: u32 = 32 - shf_bits + 127;
        // sign 1bit
        let mut ret = 0x80000000u32 & x as u32;
        // E 8bit
        ret |= e << 23;
        // frac 23
        ret |= f >> 9;
        ret
    }
    let ret = int2float(x);
    println!("{}", f32::from_bits(ret));
}

#[test]
fn _49() {
    // 2^24 + 1
    let u = 0x01000001u32;
    println!("{:0>8X}", u);
    let f = u as f32;
    println!("{:0>8X}", f.to_bits());
}

#[test]
fn _52() {
    use super::f8::*;

    let arr: [u8; 5] = [0b00110000, 0b01011110, 0b00101001, 0b01101111, 0b00000001];
    println!("     格式A             |     格式B     ");
    println!("   位   |      值      |   位   |     值");
    for e in arr {
        let eav = F8a::from_bits(e);
        let ebv = eav.to_f8b();
        println!("{:0>8b}|{}|{:0>8b}|{}", e, eav, ebv.bits(), ebv);
    }
}

#[test]
fn _53() {
    // 自定义双精度浮点数的 POS_INFINITY, NEG_INFINITY, NEG_ZERO
    // 双精度能够表示的最大的有限数, 提示：f64能表示的最大的有限数约等于1.8x10^308

    //实际上加最大浮点数（非无穷）的小数最后一位就行了，但是这毕竟是约数，加大点也行。
    let pos_infinity: f64 = 1.8 * 10f64.powi(308) + 0.5;
    println!("POS_INFINITY: {}", pos_infinity,);

    let neg_infinity: f64 = -pos_infinity;
    println!("NEG_INFINITY: {}", neg_infinity);

    let neg_zero: f64 = -0f64;
    println!("NEG_ZERO: {}", neg_zero);
    assert_eq!(f64::INFINITY, pos_infinity);
    assert_eq!(f64::NEG_INFINITY, neg_infinity);
}

// homeworks
// _55() none
// _56() none
// _57() none

#[test]
fn _58() {
    // 判断系统是大端还是小端
    fn is_little_endian() -> bool {
        let a = 0x0100u16;
        let first = unsafe {
            let a_ptr = std::mem::transmute::<&u16, *const u8>(&a);
            *a_ptr
        };

        first == 0
    }

    assert!(is_little_endian());
}

#[test]
fn _59() {
    // 将x的最低有效字节和y的其余字节组合生成新的数字
    fn mix(x: u32, y: u32) -> u32 {
        let x_last_byte = x & 0x000000ff;
        let y_other_bytes = y & 0xffffff00;
        x_last_byte | y_other_bytes
    }

    assert_eq!(0x765432EF, mix(0x89ABCDEF, 0x76543210));
}

#[test]
fn _60() {
    fn replace_byte(x: u32, i: u32, b: u8) -> u32 {
        let mask_code = 0xffu32 << (i * 8);
        let b_u32_shift = (b as u32) << (i * 8);
        let x_masked = x & !mask_code;
        x_masked | b_u32_shift
    }
    assert_eq!(0x12AB5678, replace_byte(0x12345678, 2, 0xAB));
    assert_eq!(0x123456AB, replace_byte(0x12345678, 0, 0xAB));
}
// 以下题目对int值的大小是不确定的，姑且使用i32替代，但是应对i8,i16都有效
#[test]
fn _61() {
    fn set_all_bits_1(x: i32) -> i32 {
        x | -1
    }
    fn set_all_bits_0(_x: i32) -> i32 {
        // _x & 0
        0
    }
    fn set_lowest_byte_1(x: i32) -> i32 {
        x | 0xff
    }
    fn set_highest_byte_0(x: i32) -> i32 {
        let w = std::mem::size_of::<i32>() << 3;
        x & !(0xff << (w - 8))
    }
    let x = 0x12345678;
    assert_eq!(-1, set_all_bits_1(x));
    assert_eq!(0, set_all_bits_0(x));
    assert_eq!(0x123456ff, set_lowest_byte_1(x));
    assert_eq!(0x00345678, set_highest_byte_0(x));
}

#[test]
fn _62() {
    fn int_shifts_are_arithmetic() -> bool {
        let x = -1;
        (x >> 1) == -1
    }

    assert!(int_shifts_are_arithmetic());
}

#[test]
fn _63() {
    fn srl(x: u32, k: i32) -> u32 {
        let xsra: u32 = (x as i32 >> k) as u32;
        // 如果x为负数的补码，则mask对应位置为1，否则全为0
        let mask = ((i32::MIN & x as i32) >> (k - 1)) as u32;
        xsra & !mask
    }

    fn sra(x: i32, k: i32) -> i32 {
        let xsrl: i32 = (x as u32 >> k) as i32;
        // 如果x为负数的补码，则mask对应位置为1，否则全为0
        let mask = (i32::MIN & x) >> (k - 1);
        xsrl | mask
    }
    let x: u32 = 0xf2345678;
    assert_eq!(srl(x, 8), x >> 8);
    let ix: i32 = -1;
    assert_eq!(sra(ix, 8), ix >> 8);
}

#[test]
fn _64() {
    // Return true when any odd bit of x equals 1;
    // Assume w=32
    fn any_odd_one(x: u32) -> bool {
        let ret = x & 0x55555555;
        ret != 0
    }
    assert!(any_odd_one(0x77777777));
    assert!(!any_odd_one(0xaaaaaaaa));
}

#[test]
fn _65() {
    /* Return true when x contains an odd number of 1s; false otherwise.
    Assume w = 32 */
    fn odd_ones(x: u32) -> bool {
        todo!()
    }
}
