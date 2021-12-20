use crate::{BCD_DIGITS_128, BIN_BITS_128};

///
pub fn bcd2bin128(bcd: &[u8; BCD_DIGITS_128]) -> u128 {
  let mut num: u128 = 0;
  let mut bcd = *bcd;
  let msb: u128 = 1 << (BIN_BITS_128 - 1);
  let mut carry = [0; BCD_DIGITS_128 - 1];
  for _ in 0..BIN_BITS_128 {
    num >>= 1;
    let mut k = BCD_DIGITS_128 - 1;
    if (bcd[k] & 0x1) == 1 {
      num |= msb;
    }
    while k > 0 {
      carry[k - 1] = (bcd[k - 1] & 0x1) << 3;
      bcd[k] = ((bcd[k] >> 1) | carry[k - 1]) & 0xF;
      if bcd[k] > 7 {
        bcd[k] -= 3;
      }
      k -= 1;
    }
    bcd[k] = (bcd[k] >> 1) & 0xF;
    if bcd[k] > 7 {
      bcd[k] -= 3;
    }
  }
  num
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bin128_1_digit() {
    assert_eq!(
      1,
      bcd2bin128(&[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
      ])
    );
  }

  #[test]
  fn test_bin128_8_digits() {
    assert_eq!(
      89999999,
      bcd2bin128(&[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 8, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }

  #[test]
  fn test_bin128_16_digits() {
    assert_eq!(
      8999999999999999,
      bcd2bin128(&[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }

  #[test]
  fn test_bin128_32_digits() {
    assert_eq!(
      89999999999999999999999999999999,
      bcd2bin128(&[
        0, 0, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }

  #[test]
  fn test_bin128_34_digits() {
    assert_eq!(
      8999999999999999999999999999999999,
      bcd2bin128(&[
        8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }
}
