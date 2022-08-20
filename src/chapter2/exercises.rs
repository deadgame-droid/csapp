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
// see _47.md

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
            let a_ptr = &a as *const u16 as *const u8;
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
    Assume w=32 */
    fn odd_ones(mut x: u32) -> bool {
        x ^= x >> 1;
        x ^= x >> 2;
        x ^= x >> 4;
        x ^= x >> 8;
        x ^= x >> 16;
        (x & 1) == 1
    }
    assert!(!odd_ones(0xffffffff));
    assert!(odd_ones(0xfffffffe));
    assert!(!odd_ones(0x11111111));
    assert!(odd_ones(0x11001110));
}

#[test]
fn _66() {
    /*
     * Generate mask indicating leftmost 1 in x. Assume w=32.
     * For example, 0xFF00 --> 0x8000, and 0x6600 --> 0x4000.
     * If x = 0, then return 0.
     */
    fn leftmost_one(mut x: u32) -> i32 {
        // x => 0b0..01..1
        x |= x >> 1;
        x |= x >> 2;
        x |= x >> 4;
        x |= x >> 8;
        x |= x >> 16;
        let y = !(x >> 1);
        (x & y) as i32
    }

    assert_eq!(0x8000, leftmost_one(0xFF00));
    assert_eq!(0x4000, leftmost_one(0x6600));
}

#[test]
fn _67() {
    // see _67.c
}

#[test]
fn _68() {
    /*
     * Mask with least signficant n bits set to 1
     * Examples: n = 6 --> 0x3F, n = 17 --> 0x1FFFF
     * Assume 1 <= n <= w
     */
    fn lower_one_mask(n: i32) -> i32 {
        let ret = -2 << (n - 1);
        !ret
    }

    assert_eq!(0x3F, lower_one_mask(6));
    assert_eq!(0x1FFFF, lower_one_mask(17));
    assert_eq!(-1, lower_one_mask(32));
}

#[test]
fn _69() {
    /*
     * Do rotaing left shift. Assume 0 <= n < w
     * Examples when x = 0x12345678 and w = 32:
     *      n=4 -> 0x23456781, n=20 -> 0x67812345
     */
    fn rotate_left(x: u32, n: i32) -> u32 {
        let w = std::mem::size_of::<u32>() * 8;
        let left = x & !(u32::MAX >> n);
        (x << n) | (left >> (w as i32 - n - 1) >> 1)
    }
    assert_eq!(0x12345678, rotate_left(0x12345678, 0));
    assert_eq!(0x23456781, rotate_left(0x12345678, 4));
    assert_eq!(0x67812345, rotate_left(0x12345678, 20));
}

#[allow(overflowing_literals)]
#[test]
fn _70() {
    /*
     * Return true when x can be represented as an n-bit, 2's-complement
     * number; false otherwise
     * Assume 1 <= n <= w
     */
    fn fits_bits(x: i32, n: i32) -> bool {
        let w = (std::mem::size_of::<i32>() << 3) as i32;
        x << (w - n) >> (w - n) == x
    }

    assert!(fits_bits(0xffffffff, 1));
    assert!(fits_bits(0xff000000, 25));
    assert!(!fits_bits(0xff000000, 24));
    assert!(fits_bits(0x000000ff, 9));
    assert!(!fits_bits(0x000000ff, 8));
}

#[allow(overflowing_literals)]
#[test]
fn _71() {
    /*
     * Declaration of data type where 4 bytes are packed
     * into an unsigned
     */

    fn xbyte(word: u32, bytenum: i32) -> i32 {
        // (word >> (bytenum << 3)) & 0xFF 错误: 符号未考虑
        (word as i32) << (24 - (bytenum << 3)) >> 24
    }
    assert_eq!(0xFFFFFFF0, xbyte(0x00F00000, 2));
    assert_eq!(0x0000000F, xbyte(0x0F000000, 3));
}

#[test]
fn _72() {
    /* Copy integer into buffer if space is available */
    /* WARNING: The following code is buggy */
    /*
     * also see _72.c
     * void copy_int(int val, void *buf, int maxbytes)  {
     *     if (maxbytes-sizeof(val) >= 0 ) {
     *         memcpy(buf, (void *) &val, sizeof(val));
     *     }
     * }
     */
    fn copy_int(val: i32, buf: &mut [u8], maxbytes: i32) {
        if maxbytes >= std::mem::size_of_val(&val) as i32 {
            buf.copy_from_slice(&val.to_le_bytes());
        }
    }

    let mut buf = [0u8; 4];
    dbg!(buf);
    let val = 0x01020304;
    let maxbytes = buf.len();
    copy_int(val, &mut buf, maxbytes as i32);
    dbg!(buf);
}
#[allow(clippy::unit_cmp)]
#[allow(clippy::short_circuit_statement)]
#[test]
fn _73() {
    /* Addition that saturates to TMin or TMax  */
    fn saturating_add(x: i32, y: i32) -> i32 {
        let mut sum = x.wrapping_add(y);
        let min = i32::MIN;
        // positive overflow
        // x > 0 && y > 0 && sum < 0
        let pos_overflow = x & min != min && y & min != min && sum & min == min;
        // negetive overflow
        // x < 0 && y < 0 && sum >=0
        let neg_overflow = x & min == min && y & min == min && sum & min != min;
        // 因为不能使用if，这个利用了一个技巧，强行使用&&和赋值的结果()，实现if的效果。
        let _ = pos_overflow && (sum = i32::MAX) == () || neg_overflow && (sum = i32::MIN) == ();
        sum
    }
    assert_eq!(i32::MIN, saturating_add(i32::MIN, -1));
    assert_eq!(i32::MAX, saturating_add(i32::MAX, 1));
}

#[test]
fn _74() {
    /*
     * Detemine whether arguments can be subtracted without overflow
     */
    fn tsub_ok(x: i32, y: i32) -> bool {
        let z: i32 = x.wrapping_sub(y);
        let min: i32 = i32::MIN;
        // positive overflow
        // x > 0 && -y > 0 && sum < 0
        let pos_overflow: bool = x & min != min && y & min == min && z & min == min;
        // negetive overflow
        // x < 0 && -y < 0 && sum >=0
        let neg_overflow: bool = x & min == min && y & min != min && z & min != min;
        !(pos_overflow || neg_overflow)
    }
    assert!(tsub_ok(0x00, 0x00));
    assert!(!tsub_ok(0, i32::MIN));
}

#[test]
fn _75() {
    // 提供的计算高w位的有符号数函数
    fn signed_high_prod(x: i32, y: i32) -> i32 {
        let prod: i64 = x as i64 * y as i64;
        (prod >> 32) as i32
    }

    fn unsigned_high_prod(x: u32, y: u32) -> u32 {
        let w: usize = std::mem::size_of::<i32>() * 8;
        let sign_x: u32 = x >> (w - 1);
        let sign_y: u32 = y >> (w - 1);
        let signed_ret: i32 = signed_high_prod(x as i32, y as i32);
        (signed_ret as u32)
            .wrapping_add(sign_x * y)
            .wrapping_add(sign_y * x)
    }

    fn another_unsigned_high_prod(x: u32, y: u32) -> u32 {
        let prod: u64 = x as u64 * y as u64;
        (prod >> 32) as u32
    }

    let x: u32 = 0x77777774;
    let y: u32 = 0x88888828;
    println!("{:X}", x as u64 * y as u64);
    println!("i32:    {:X}", signed_high_prod(x as i32, y as i32));
    println!("a-u32:  {:X}", another_unsigned_high_prod(x, y));
    println!("u32 :   {:X}", unsigned_high_prod(x, y));
}

#[test]
fn _76() {
    // see c
}

#[test]
fn _77() {
    let x = 1;
    // x * 17
    assert_eq!(17, (x << 4) + 1);
    // x * -7
    assert_eq!(-7, x - (x << 3));
    // x * 60
    assert_eq!(60, (x << 6) - (x << 2));
    // x * -112
    assert_eq!(-112, (x << 4) - (x << 7));
}

#[test]
fn _78() {
    // Assume 0 <= k < w-1
    // 负数向零舍入
    fn divide_power2(mut x: i32, k: i32) -> i32 {
        let is_neg = x & i32::MIN == i32::MIN;
        if is_neg {
            x += (1 << k) - 1;
        }
        x >> k
    }
    let x = i32::MIN + 7;

    assert_eq!(x / 2, divide_power2(x, 1));
    assert_eq!(x / 4, divide_power2(x, 2));
}

#[allow(overflowing_literals)]
#[test]
fn _79() {
    fn mul3div4(x: i32) -> i32 {
        let is_neg = x & i32::MIN == i32::MIN;
        let mut ret = (x << 1) + x;
        if is_neg {
            ret += 3;
        }
        ret >> 2
    }
    let x = 0x87654321;
    assert!(mul3div4(x) == x.wrapping_mul(3).wrapping_div(4));
}

#[test]
fn _80() {
    // 3/4 * x, no overflow, round to zero
    fn threefourths(x: i32) -> i32 {
        let is_neg = x & i32::MIN == i32::MIN;
        if is_neg {
            let offset = (((x + 1) >> 1) & 1) & (x & 1);
            ((x + 1) >> 1) + ((x + 3) >> 2) - offset
        } else {
            let offset = ((x >> 1) & 1) & (x & 1);
            (x >> 1) + (x >> 2) + offset
        }
    }
    assert_eq!(threefourths(8), 6);
    assert_eq!(threefourths(9), 6);
    assert_eq!(threefourths(10), 7);
    assert_eq!(threefourths(11), 8);
    assert_eq!(threefourths(12), 9);

    assert_eq!(threefourths(-8), -6);
    assert_eq!(threefourths(-9), -6);
    assert_eq!(threefourths(-10), -7);
    assert_eq!(threefourths(-11), -8);
    assert_eq!(threefourths(-12), -9);
}

#[allow(overflowing_literals)]
#[test]
fn _81() {
    let j = 4;
    let k = 8;
    assert_eq!(0xffffff00, -1 << k);
    assert_eq!(0x00000ff0, !(-1 << (k)) << j);
}

#[test]
fn _82() {
    // see book page93
}

#[test]
fn _83() {
    // see book page93
}

#[test]
fn _84() {
    // x less than y
    fn float_le(x: f32, y: f32) -> bool {
        let ux: u32 = x.to_bits();
        let uy: u32 = y.to_bits();

        // Get the sign bits
        let sx: u32 = ux >> 31;
        let sy: u32 = uy >> 31;

        if ux << 1 == 0 && 0 == uy << 1 {
            // both zero, -0 or 0
            true
        } else if sx == sy {
            if sx == 0 {
                ux <= uy
            } else {
                ux >= uy
            }
        } else {
            sx > sy
        }
    }

    assert!(float_le(-0f32, 0f32));
    assert!(float_le(-1.2, -1.0));
    assert!(float_le(-1.0, 1.0));
    assert!(float_le(1.0, 1.2));
}

#[test]
fn _85() {
    // see book page93
}

#[test]
fn _86() {
    // see book page94
}

#[test]
fn _87() {
    // see book page94
}

#[test]
fn _88() {
    // see book page95
}

#[test]
fn _89() {
    // see book page95
}

#[test]
fn _90() {
    fn fpwr2(x: i32) -> f32 {
        /* Result exponent and fraction */
        let exp: u32;
        let frac: u32;

        if x < -149 {
            /* Too small. Return 0.0 */
            exp = 0;
            frac = 0;
        } else if x < -126 {
            /* Denormalized result */
            exp = 0;
            frac = 1 << (149 + x);
        } else if x < 128 {
            /* Normalized result */
            exp = (x + 127) as u32;
            frac = 0;
        } else {
            /* Too big. Return +oo */
            exp = 255;
            frac = 0;
        }
        let u = exp << 23 | frac;
        f32::from_bits(u)
    }

    for i in -150..129 {
        let left = 2f32.powi(i);
        let right = fpwr2(i);
        if left != right {
            println!("{} != {}", left, right);
            dbg!(i);
        }
    }
}
#[test]
fn _91() {
    // see book page96
}

#[test]
fn _92() {
    // Compute -f. If f is NaN, then return f.
    fn float_negate(f: u32) -> u32 {
        let is_nan = (f & 0x7f800000 == 0x7f800000) && (f & 0x007fffff != 0);
        if is_nan {
            f
        } else {
            let sign_bits = f ^ 0x80000000;
            let other_bits = f & 0x7fffffff;
            sign_bits | other_bits
        }
    }
    // test
    let x = 0x7f800000;
    assert_eq!((-f32::from_bits(x)).to_bits(), float_negate(x));
}

#[test]
fn _93() {
    // Compute -f. If f is NaN, then return f.
    fn float_absval(f: u32) -> u32 {
        let is_nan = (f & 0x7f800000 == 0x7f800000) && (f & 0x007fffff != 0);
        if is_nan {
            f
        } else {
            f & 0x7fffffff
        }
    }
    // test
    let x = 0xff001000;
    assert_eq!((f32::from_bits(x).abs()).to_bits(), float_absval(x));
}

#[test]
fn _94() {
    use super::float_bits::*;
    // Compute -f. If f is NaN, then return f.
    fn float_twice(f: u32) -> u32 {
        if is_nan(f) {
            f
        } else if is_denorm(f) {
            let frac_bits = f & 0x007fffff;
            (frac_bits << 1) | (f & 0xff000000)
        } else if is_norm(f) {
            // 规格化数直接将阶码位进1，若已达到最大值则进阶为无穷大
            let exp_bits = f & 0x7f800000;
            if exp_bits == 0x7f000000 {
                // 进阶为无穷大
                let sign_bit = f & 0x80000000;
                sign_bit | 0x7f800000
            } else {
                (exp_bits + 0x00800000) | (f & 0x807fffff)
            }
        } else {
            // 无穷则返回无穷
            f
        }
    }
    // test
    let mut x = 0x00000000;
    assert_eq!((f32::from_bits(x) * 2.0).to_bits(), float_twice(x));
    x = 0x00400000;
    assert_eq!((f32::from_bits(x) * 2.0).to_bits(), float_twice(x));
    x = 0x004fffff;
    assert_eq!((f32::from_bits(x) * 2.0).to_bits(), float_twice(x));
    x = 0x007fffff;
    assert_eq!((f32::from_bits(x) * 2.0).to_bits(), float_twice(x));
}

#[test]
fn _95() {
    use super::float_bits::*;
    // Compute -f. If f is NaN, then return f.
    fn float_half(f: u32) -> u32 {
        if is_nan(f) {
            f
        } else if is_denorm(f) {
            let frac_bits = f & 0x007fffff;
            // 向偶数舍入
            let trancate = frac_bits & 1;
            let last_bit = (frac_bits >> 1) & 1;
            ((frac_bits >> 1) + (trancate & last_bit)) | (f & 0xff800000)
        } else if is_norm(f) {
            // 规格化数直接将阶码位减1，若阶码已达到最小值则变为非规格化数
            let exp_bits = f & 0x7f800000;
            if exp_bits == 0x00800000 {
                // 降为非规格化数
                f & 0xff00000 | ((f & 0x00fffff) >> 1)
            } else {
                (exp_bits - 0x00800000) | (f & 0x807fffff)
            }
        } else {
            // 无穷则返回无穷
            f
        }
    }
    // test
    let mut x = 0x00000000;
    assert_eq!((f32::from_bits(x) * 0.5).to_bits(), float_half(x));
    x = 0x00400000;
    assert_eq!((f32::from_bits(x) * 0.5).to_bits(), float_half(x));
    x = 0x004fffff;
    assert_eq!((f32::from_bits(x) * 0.5).to_bits(), float_half(x));
    x = 0x007fffff;
    assert_eq!((f32::from_bits(x) * 0.5).to_bits(), float_half(x));
}

#[test]
fn _96() {
    use super::float_bits::*;
    fn float_f2i(f: u32) -> i32 {
        if is_nan(f) || is_denorm(f) {
            0
        } else if is_norm(f) {
            let sign = f >> 31;
            let exp = f >> 23 & 0xff;
            if exp < 127 {
                0
            } else {
                let e = exp - 127;
                let frac = f & 0x7fffff;

                let ret = if e <= 23 {
                    (1 << e) + (frac >> (23 - e))
                } else {
                    (frac + 0x800000) << e
                };

                if sign == 0 && ret <= i32::MAX as u32 {
                    ret as i32
                } else if sign == 1 && ret <= i32::MAX as u32 + 1 {
                    (!ret + 1) as i32
                } else {
                    #[allow(overflowing_literals)]
                    0x80000000
                }
            }
        } else {
            let sign = f >> 31;
            if sign == 0 {
                i32::MAX
            } else {
                i32::MIN
            }
        }
    }
    let x = 0x2f800000;
    assert_eq!(float_f2i(x), f32::from_bits(x) as i32);
}

#[test]
fn _97() {
    fn float_i2f(i: i32) -> u32 {
        if i == 0 {
            return 0;
        }
        let sign = (i >> 31) as u32;
        let i_abs = if sign == 1 { (!i + 1) as u32 } else { i as u32 };
        let mut one_offset: u32 = 0;
        let mut frac = i_abs;
        while frac & 0x80000000 == 0 {
            frac <<= 1;
            one_offset += 1;
        }
        // 因为非规格化数的精度为23+1(隐式)，第二位1为frac的第一位, 所以在此多位移一次
        frac <<= 1;
        one_offset += 1;
        // 如果i的有效位超过能保存的24位，则需要向偶数舍入
        if one_offset < 9 {
            let frac_last_bit = frac >> 9 & 1;
            let trancate = frac << 23 >> 23;
            if trancate > 2u32.pow(9 - one_offset - 2)
                || (trancate == 2u32.pow(9 - one_offset - 2) && frac_last_bit == 1)
            {
                frac = (frac >> 9) + 1;
            } else {
                frac >>= 9;
            }
        } else {
            frac >>= 9;
        }
        let exp = 32 - one_offset + 127;
        // 相加是因为frac在舍入时，可以溢出23位，此时只需将舍入的部分与exp相加
        sign << 31 | ((exp << 23) + frac)
    }

    assert_eq!(0f32.to_bits(), float_i2f(0));
    assert_eq!(8f32.to_bits(), float_i2f(8));
}
