//! Library for converting "standard width" characters to fullwidth and vice versa

#![warn(missing_docs, clippy::pedantic)]

/// Extension trait for char fullwidth conversion
pub trait CharExt {
    /// Is this character fullwidth?
    #[expect(
        clippy::wrong_self_convention,
        reason = "It's fine to take self by value here"
    )]
    fn is_fw(self) -> bool;
    /// Convert from standard to full width
    #[must_use]
    fn to_fw(self) -> Self;
    /// Convert from full to standard width
    #[must_use]
    fn to_sw(self) -> Self;
}

impl CharExt for char {
    fn is_fw(self) -> bool {
        matches!(self, 'Ａ'..='Ｚ' | 'ａ'..='ｚ' | '０'..='９' | '｜' | '／' | FW_SPACE)
    }
    fn to_fw(self) -> Self {
        match self {
            'A'..='Z' => char::from_u32(self as u32 + FW_UC_ADD).unwrap(),
            'a'..='z' => char::from_u32(self as u32 + FW_LC_ADD).unwrap(),
            '0'..='9' => char::from_u32(self as u32 + FW_NUM_ADD).unwrap(),
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
            '０'..='９' => char::from_u32(self as u32 - FW_NUM_ADD).unwrap(),
            '｜' => '|',
            '／' => '/',
            FW_SPACE => ' ',
            _ => self,
        }
    }
}

/// Extension trait for string fullwidth conversion
pub trait StrExt {
    /// Whether this string has any fullwidth characters
    fn has_fw(&self) -> bool;
    /// Convert from standard to full width
    #[must_use]
    fn to_fw(&self) -> String;
    /// Convert from full to standard width
    #[must_use]
    fn to_sw(&self) -> String;
}

impl<T: AsRef<str> + ?Sized> StrExt for T {
    fn has_fw(&self) -> bool {
        self.as_ref().chars().any(CharExt::is_fw)
    }

    fn to_fw(&self) -> String {
        self.as_ref().chars().map(CharExt::to_fw).collect()
    }

    fn to_sw(&self) -> String {
        self.as_ref().chars().map(CharExt::to_sw).collect()
    }
}

const FW_NUM_OFFSET: u32 = 0xFF10;
const FW_UC_OFFSET: u32 = 0xFF21;
const FW_LC_OFFSET: u32 = 0xFF41;
const FW_NUM_ADD: u32 = FW_NUM_OFFSET - '0' as u32;
const FW_LC_ADD: u32 = FW_UC_OFFSET - 'A' as u32;
const FW_UC_ADD: u32 = FW_LC_OFFSET - 'a' as u32;
const FW_SPACE: char = char::from_u32(0x3000).unwrap();

#[test]
fn test_sw_to_fw() {
    assert_eq!("hello world".to_fw(), "ｈｅｌｌｏ　ｗｏｒｌｄ");
    assert_eq!(
        "hello 1234 world".to_fw(),
        "ｈｅｌｌｏ　１２３４　ｗｏｒｌｄ"
    );
}

#[test]
fn test_fw_to_sw() {
    assert_eq!("ｈｅｌｌｏ　ｗｏｒｌｄ".to_sw(), "hello world");
    assert_eq!(
        "ｈｅｌｌｏ　１２３４　ｗｏｒｌｄ".to_sw(),
        "hello 1234 world"
    );
}

#[test]
fn test_has_fw() {
    assert!(!"hello world".has_fw());
    assert!("ｈｅｌｌｏ　ｗｏｒｌｄ".has_fw());
    // Normal looking string with fullwidth space in middle
    assert!("hello　world".has_fw());
    // Numbers
    assert!("２".has_fw());
    assert!("hello １２３４ world".has_fw());
}
