/// Merge two numbers with offset, this way you can merge two u8's that are behind
/// each other into their corresponding u16 value.
fn merge_two_numbers_with_offset_as<T, R>(n1: T, n2: T, offset: usize) -> R {
    (n1 << offset) | n2
}

/// Merge two succeeding u8's into a u16.
pub fn merge_u8_as_u16(n1: u8, n2: u8) -> u16 {
    merge_two_numbers_with_offset_as(n1, n2, 8)
}

/// Merge two succeeding u16's into a u32.
pub fn merge_u16_as_u32(n1: u16, n2: u16) -> u32 {
    merge_two_numbers_with_offset_as(n1, n2, 16)
}
