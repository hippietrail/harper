use is_macro::Is;
use serde::{Deserialize, Serialize};

#[derive(Debug, Is, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
pub enum Currency {
    // $
    Dollar,
    // €
    Euro,
    // ₽
    Ruble,
    // ₺
    Lira,
    // £
    Pound,
    // ¥
    Yen,
    // ฿
    Baht,
    // ₩
    Won,
    // ₭,
    Kip,
}

impl Currency {
    pub fn from_char(c: char) -> Option<Self> {
        let cur = match c {
            '$' => Self::Dollar,
            '€' => Self::Euro,
            '₽' => Self::Ruble,
            '₺' => Self::Lira,
            '£' => Self::Pound,
            '¥' => Self::Yen,
            '฿' => Self::Baht,
            '₩' => Self::Won,
            '₭' => Self::Kip,
            _ => return None,
        };

        Some(cur)
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Dollar => '$',
            Self::Euro => '€',
            Self::Ruble => '₽',
            Self::Lira => '₺',
            Self::Pound => '£',
            Self::Yen => '¥',
            Self::Baht => '฿',
            Self::Won => '₩',
            Self::Kip => '₭',
        }
    }
}
