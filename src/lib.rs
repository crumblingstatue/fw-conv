/// Standard to fullwidth
pub fn sw_to_fw(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars() {
        let out_ch = match ch {
            'A'..='Z' => char::from_u32(ch as u32 + FW_UC_ADD).unwrap(),
            'a'..='z' => char::from_u32(ch as u32 + FW_LC_ADD).unwrap(),
            '|' => '｜',
            '/' => '／',
            ' ' => char::from_u32(0x3000).unwrap(),
            _ => ch,
        };
        out.push(out_ch);
    }
    out
}

const FW_UC_OFFSET: u32 = 0xFF21;
const FW_LC_OFFSET: u32 = 0xFF41;
const FW_LC_ADD: u32 = FW_UC_OFFSET - 'A' as u32;
const FW_UC_ADD: u32 = FW_LC_OFFSET - 'a' as u32;
