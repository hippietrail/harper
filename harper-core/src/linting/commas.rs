use is_macro::Is;
use serde::{Deserialize, Serialize};

use crate::Number;

#[derive(Debug, Is, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
pub enum AsianComma {
    // 、
    FullwidthCommaU3001,  
    // ，
    IdeographicCommaUFF0C,
}

impl AsianComma {
    pub fn from_char(c: char) -> Option<Self> {
        let comma = match c {
            '、' => Self::FullwidthCommaU3001,
            '，' => Self::IdeographicCommaUFF0C,
            _ => return None,
        };

        Some(comma)
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::FullwidthCommaU3001 => '、',
            Self::IdeographicCommaUFF0C => '，',
        }
    }
}
