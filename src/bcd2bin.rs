use crate::{BCD_DIGITS_128, BIN_BITS_128};

const MSB: u128 = 1 << 127;

///
pub fn bcd2bin128(b: &[u8; BCD_DIGITS_128]) -> u128 {
  let mut num: u128 = 0;
  let mut u = 0;
  let mut digits = b.clone();
  for i in 0..BCD_DIGITS_128 {
    if digits[i] != 0 {
      break;
    }
    u += 1;
  }
  for _ in 0..BIN_BITS_128 {
    num >>= 1;
    let mut k = BCD_DIGITS_128 - 1;
    if (digits[k] & 0x1) == 1 {
      num |= MSB;
    }
    while k > u {
      if (digits[k - 1] & 0x1) > 0 {
        digits[k] = (digits[k] >> 1) | 0x8;
      } else {
        digits[k] >>= 1;
      }
      if digits[k] > 7 {
        digits[k] -= 3;
      }
      k -= 1;
    }
    digits[k] >>= 1;
    if digits[k] > 7 {
      digits[k] -= 3;
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
      bcd2bin128(&mut [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
      ])
    );
  }

  #[test]
  fn test_bin128_8_digits() {
    assert_eq!(
      89999999,
      bcd2bin128(&mut [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 8, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }

  #[test]
  fn test_bin128_16_digits() {
    assert_eq!(
      8999999999999999,
      bcd2bin128(&mut [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }

  #[test]
  fn test_bin128_32_digits() {
    assert_eq!(
      89999999999999999999999999999999,
      bcd2bin128(&mut [
        0, 0, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }

  #[test]
  fn test_bin128_34_digits() {
    assert_eq!(
      8999999999999999999999999999999999,
      bcd2bin128(&mut [
        8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      ])
    );
  }
}
