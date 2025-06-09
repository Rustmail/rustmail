pub fn hex_string_to_int(hex_string: &str) -> u64 {
    u64::from_str_radix(hex_string, 16).unwrap_or_else(|_| 0)
}
