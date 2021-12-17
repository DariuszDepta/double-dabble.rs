///
pub fn bin2bcd(bin: u128, bcd: &mut [u8; 34]) {
  let mask: u128 = 1 << 127;
  let mut number = bin;
  let mut flag = false;
  for _ in 0..128 {
    let bit_mask = if (number & mask) > 0 {
      flag = true;
      0x1
    } else {
      0x0
    };
    if flag {
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
      bcd[j] |= bit_mask;
    }
    number <<= 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1_digit() {
    let mut bcd = [0; 34];
    bin2bcd(1, &mut bcd);
    assert_eq!("[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]", format!("{:?}", bcd));
  }

  #[test]
  fn test_16_digit() {
    let mut bcd = [0; 34];
    bin2bcd(8999999999999999, &mut bcd);
    assert_eq!("[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]", format!("{:?}", bcd));
  }

  #[test]
  fn test_34_digits() {
    let mut bcd = [0; 34];
    bin2bcd(8999999999999999999999999999999999, &mut bcd);
    assert_eq!("[8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]", format!("{:?}", bcd));
  }
}
