use std::ops::{BitOr, Shl};

pub const JUMP_FLAG: u8 = 0xC0;

/// Merge two numbers with offset, this way you can merge two u8's that are behind
/// each other into their corresponding u16 value.
fn merge_two_numbers_with_offset_as<T, R>(n1: T, n2: T, offset: usize) -> R
    where T: Into<R>,
          R: BitOr<Output=R> + Shl<usize, Output=R> {
    (n1.into() << offset) | n2.into()
}

/// Merge two succeeding u8's into a u16.
pub fn merge_u8_as_u16(n1: u8, n2: u8) -> u16 {
    merge_two_numbers_with_offset_as(n1, n2, 8)
}

/// Merge two succeeding u16's into a u32.
pub fn merge_u16_as_u32(n1: u16, n2: u16) -> u32 {
    merge_two_numbers_with_offset_as(n1, n2, 16)
}

// Check whether a flag is set in a value.
pub fn has_flag(value: u8, flag: u8) -> bool {
    (value & flag) == flag
}

// Direct a u16 into two u8's.
pub fn split_u16_as_u8s(n: u16) -> [u8; 2] {
    [
        ((n >> 8) & 0xFF) as u8,
        (n & 0xFF) as u8,
    ]
}

// Direct a u32 into four u8's.
pub fn split_32_as_u8s(n: u32) -> [u8; 4] {
    // Note: might want to utilize the split_u16_as_u8s function here.
    [
        ((n >> 24) & 0xFF) as u8,
        ((n >> 16) & 0xFF) as u8,
        ((n >> 8) & 0xFF) as u8,
        (n & 0xFF) as u8,
    ]
}
