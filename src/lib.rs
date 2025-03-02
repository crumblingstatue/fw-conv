//! Library for converting "standard width" characters to fullwidth and vice versa

#![warn(missing_docs, clippy::pedantic)]

/// Extension trait for char fullwidth conversion
pub trait CharExt {
    /// Convert from standard to full width
    #[must_use]
    fn to_fw(self) -> Self;
    /// Convert from full to standard width
    #[must_use]
    fn to_sw(self) -> Self;
}

impl CharExt for char {
    fn to_fw(self) -> Self {
        match self {
            'A'..='Z' => char::from_u32(self as u32 + FW_UC_ADD).unwrap(),
            'a'..='z' => char::from_u32(self as u32 + FW_LC_ADD).unwrap(),
            '|' => '｜',
            '/' => '／',
            ' ' => FW_SPACE,
            _ => self,
        }
    }

    fn to_sw(self) -> Self {
        match self {
            'Ａ'..='Ｚ' => char::from_u32(self as u32 - FW_UC_ADD).unwrap(),
            'ａ'..='ｚ' => char::from_u32(self as u32 - FW_LC_ADD).unwrap(),
            '｜' => '|',
            '／' => '/',
            FW_SPACE => ' ',
            _ => self,
        }
    }
}

/// Extension trait for string fullwidth conversion
pub trait StrExt {
    /// Convert from standard to full width
    #[must_use]
    fn to_fw(&self) -> String;
    /// Convert from full to standard width
    #[must_use]
    fn to_sw(&self) -> String;
}

impl<T: AsRef<str> + ?Sized> StrExt for T {
    fn to_fw(&self) -> String {
        self.as_ref().chars().map(CharExt::to_fw).collect()
    }

    fn to_sw(&self) -> String {
        self.as_ref().chars().map(CharExt::to_sw).collect()
    }
}

const FW_UC_OFFSET: u32 = 0xFF21;
const FW_LC_OFFSET: u32 = 0xFF41;
const FW_LC_ADD: u32 = FW_UC_OFFSET - 'A' as u32;
const FW_UC_ADD: u32 = FW_LC_OFFSET - 'a' as u32;
const FW_SPACE: char = char::from_u32(0x3000).unwrap();

#[test]
fn test_sw_to_fw() {
    assert_eq!("hello world".to_fw(), "ｈｅｌｌｏ　ｗｏｒｌｄ");
}

#[test]
fn test_fw_to_sw() {
    assert_eq!("ｈｅｌｌｏ　ｗｏｒｌｄ".to_sw(), "hello world");
}
