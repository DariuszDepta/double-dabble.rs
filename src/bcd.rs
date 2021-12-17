///
pub const BCD_DIGITS_128: usize = 34;
///
const BIN_BITS_128: usize = 128;

///
pub fn bin2bcd128(bin: u128, bcd: &mut [u8; BCD_DIGITS_128]) {
  let mask: u128 = 1 << (BIN_BITS_128 - 1);
  let mut bit_mask;
  let mut carry = [0; BCD_DIGITS_128];
  let mut num = bin;
  let mut flag = false;
  for _ in 0..BIN_BITS_128 {
    bit_mask = if (num & mask) > 0 {
      flag = true;
      0x1
    } else {
      0x0
    };
    if flag {
      let mut k = BCD_DIGITS_128 - 1;
      if bcd[k] > 4 {
        bcd[k] += 3;
      }
      carry[k] = (bcd[k] >> 3) & 0xF;
      bcd[k] = ((bcd[k] << 1) & 0xF) | bit_mask;
      while k > 0 {
        k -= 1;
        if bcd[k] > 4 {
          bcd[k] += 3;
        }
        carry[k] = (bcd[k] >> 3) & 0xF;
        bcd[k] = ((bcd[k] << 1) | carry[k + 1]) & 0xF;
      }
    }
    num <<= 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bcd128_1_digit() {
    let mut bcd = [0; BCD_DIGITS_128];
    bin2bcd128(1, &mut bcd);
    assert_eq!("[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]", format!("{:?}", bcd));
  }

  #[test]
  fn test_bcd128_8_digits() {
    let mut bcd = [0; BCD_DIGITS_128];
    bin2bcd128(89999999, &mut bcd);
    assert_eq!("[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 9, 9, 9, 9, 9, 9]", format!("{:?}", bcd));
  }

  #[test]
  fn test_bcd128_16_digits() {
    let mut bcd = [0; BCD_DIGITS_128];
    bin2bcd128(8999999999999999, &mut bcd);
    assert_eq!("[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]", format!("{:?}", bcd));
  }

  #[test]
  fn test_bcd128_32_digits() {
    let mut bcd = [0; BCD_DIGITS_128];
    bin2bcd128(89999999999999999999999999999999, &mut bcd);
    assert_eq!("[0, 0, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]", format!("{:?}", bcd));
  }

  #[test]
  fn test_bcd128_34_digits() {
    let mut bcd = [0; BCD_DIGITS_128];
    bin2bcd128(8999999999999999999999999999999999, &mut bcd);
    assert_eq!("[8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]", format!("{:?}", bcd));
  }
}
