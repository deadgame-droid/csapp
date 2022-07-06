#![allow(unused)]
use std::fmt::Display;

// 8bits float, 3bits for E, 4bits for frac
pub struct F8a {
    bits: u8,
}
impl F8a {
    pub fn from_bits(bits: u8) -> Self {
        Self { bits }
    }
    pub fn to_f8b(&self) -> F8b {
        let mut bits = self.bits;
        // 检查小数部分舍入情况，像舍入到偶数
        if (bits & 3) == 3 {
            // 尾数0b******11的情况才需要进1，其他情况之后移位刚好截去
            bits += 1;
        }
        // 阶码 3->4
        bits >>= 1;
        bits += 32;
        F8b { bits }
    }
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl Display for F8a {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bits = self.bits;
        let sign = if (bits & 0b10000000) != 0 { '-' } else { ' ' };
        let e = ((bits & 0b01110000) >> 4) as i8 - 3;
        // 小数部分用分数表示，因为还未学习到浮点数的运算
        let frac = (bits & 0b00001111) + 16;
        write!(f, "{}{:>2}/16 x 2^{:<3}", sign, frac, e)
    }
}

// 8bits float, 4bits for E, 3bits for frac
pub struct F8b {
    bits: u8,
}
impl F8b {
    pub fn from_bits(bits: u8) -> Self {
        Self { bits }
    }
    pub fn bits(&self) -> u8 {
        self.bits
    }
}

impl Display for F8b {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bits = self.bits;
        let sign = if (bits & 0b10000000) != 0 { '-' } else { ' ' };
        let e = ((bits & 0b01111000) >> 3) as i8 - 7;
        // 小数部分用分数表示，因为还未学习到浮点数的运算
        let frac = (bits & 0b00000111) + 8;
        write!(f, "{}{:>2}/8 x 2^{:<3}", sign, frac, e)
    }
}
