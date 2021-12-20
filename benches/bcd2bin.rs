#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
  use bcd::{bcd2bin128, BCD_DIGITS_128};
  use test::Bencher;

  #[bench]
  fn bench_bcd2bin_128_01_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_128] = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];
    assert_eq!(1, bcd2bin128(&mut bcd));
    b.iter(|| {
      bcd2bin128(&mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin_128_08_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_128] = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 8, 9, 9, 9, 9, 9, 9, 9,
    ];
    assert_eq!(89999999, bcd2bin128(&mut bcd));
    b.iter(|| {
      bcd2bin128(&mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin_128_16_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_128] = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 9, 9, 9, 9,
      9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    ];
    assert_eq!(8999999999999999, bcd2bin128(&mut bcd));
    b.iter(|| {
      bcd2bin128(&mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin_128_32_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_128] = [
      0, 0, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    ];
    assert_eq!(89999999999999999999999999999999, bcd2bin128(&mut bcd));
    b.iter(|| {
      bcd2bin128(&mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin_128_34_digits(b: &mut Bencher) {
    let mut bcd: [u8; BCD_DIGITS_128] = [
      8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    ];
    assert_eq!(8999999999999999999999999999999999, bcd2bin128(&mut bcd));
    b.iter(|| {
      bcd2bin128(&mut bcd);
    });
  }
}
