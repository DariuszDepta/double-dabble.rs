///
pub fn bin2bcd(bin: u128, bcd: &mut [u8; 34]) {
  let mut n = bin;
  let mask: u128 = 1 << 127;
  for _ in 0..128 {
    for value in bcd.iter_mut() {
      if *value > 4 {
        *value += 3;
      }
    }
    let mut j = 0;
    while j < 33 {
      bcd[j] = ((bcd[j] << 1) & 0xF) | (bcd[j + 1] & 0x8) >> 3;
      j += 1;
    }
    bcd[j] = (bcd[j] << 1) & 0xF;
    if (n & mask) > 0 {
      bcd[j] |= 0x1;
    }
    n <<= 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_34_digits() {
    let mut bcd = [0_u8; 34];
    bin2bcd(8999999999999999999999999999999999, &mut bcd);
    assert_eq!("[8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]", format!("{:?}", bcd));
  }
}
