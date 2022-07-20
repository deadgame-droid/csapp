#![allow(unused)]
/// 使用f32的位级表示即u32，并直接使用u32来进行f32相关运算
pub fn is_nan(f: u32) -> bool {
    let exp_bits = f & 0x7f800000;
    exp_bits == 0x7f800000 && f & 0x007fffff != 0
}

pub fn is_denorm(f: u32) -> bool {
    let exp_bits = f & 0x7f800000;
    exp_bits == 0
}

pub fn is_norm(f: u32) -> bool {
    let exp_bits = f & 0x7f800000;
    exp_bits > 0 && exp_bits < 0x7f800000
}

pub fn is_inf(f: u32) -> bool {
    let exp_bits = f & 0x7f800000;
    let frac_bits = f & 0x007fffff;
    exp_bits == 0x7f800000 && frac_bits == 0
}
