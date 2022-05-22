use std::ops::{BitAnd, BitOr, BitXor};
#[derive(Debug)]
pub enum Color {
    Black,
    Blue,
    Green,
    BlueGreen,
    Red,
    RedPurple,
    Yellow,
    White,
}

impl Color {
    fn to_rbg(&self) -> u8 {
        match self {
            Color::Black => 0,
            Color::Blue => 1,
            Color::Green => 2,
            Color::BlueGreen => 3,
            Color::Red => 4,
            Color::RedPurple => 5,
            Color::Yellow => 6,
            Color::White => 7,
        }
    }
}

impl From<u8> for Color {
    fn from(rbg: u8) -> Self {
        match rbg {
            0 => Color::Black,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::BlueGreen,
            4 => Color::Red,
            5 => Color::RedPurple,
            6 => Color::Yellow,
            7 => Color::White,
            _ => panic!("unsupported rbg"),
        }
    }
}

impl BitAnd for Color {
    type Output = Color;

    fn bitand(self, rhs: Self) -> Self::Output {
        let res = self.to_rbg() & rhs.to_rbg();
        Self::from(res)
    }
}

impl BitOr for Color {
    type Output = Color;

    fn bitor(self, rhs: Self) -> Self::Output {
        let res = self.to_rbg() | rhs.to_rbg();
        Self::from(res)
    }
}

impl BitXor for Color {
    type Output = Color;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let res = self.to_rbg() ^ rhs.to_rbg();
        Self::from(res)
    }
}
