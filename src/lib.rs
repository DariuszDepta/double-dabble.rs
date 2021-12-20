mod bcd2bin;
mod bin2bcd;

///
pub const BCD_DIGITS_128: usize = 34;
///
const BIN_BITS_128: usize = 128;

pub use crate::bcd2bin::bcd2bin128;
pub use crate::bin2bcd::{bin2bcd128, bin2bcd8, BCD_DIGITS_8};
