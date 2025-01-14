use is_macro::Is;
use serde::{Deserialize, Serialize};

use crate::NumberSuffix;

#[derive(Debug, Is, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
pub enum Currency {
    // $
    Dollar,
    // ¢
    Cent,
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
            '¢' => Self::Cent,
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
            Self::Cent => '¢',
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

    /// Format an amount of the specific currency.
    pub fn format_amount(&self, value: f64, suffix: Option<NumberSuffix>) -> String {
        let c = self.to_char();

        let mut amount = value.to_string();

        if let Some(suffix) = suffix {
            amount.extend(suffix.to_chars());
        }

        match self {
            Currency::Dollar => format!("{}{amount}", c),
            Currency::Cent => format!("{amount}{}", c),
            Currency::Euro => format!("{}{amount}", c),
            Currency::Ruble => format!("{amount} {}", c),
            Currency::Lira => format!("{amount} {}", c),
            Currency::Pound => format!("{}{amount}", c),
            Currency::Yen => format!("{} {amount}", c),
            Currency::Baht => format!("{amount} {}", c),
            Currency::Won => format!("{} {amount}", c),
            Currency::Kip => format!("{}{amount}", c),
        }
    }
}
