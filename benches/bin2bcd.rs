#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
  use bcd::{bin2bcd128, BCD_DIGITS_128};
  use test::Bencher;

  #[bench]
  fn bench_bcd128_1_digit(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(1, &mut bcd);
    });
  }

  #[bench]
  fn bench_bcd128_8_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(89999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_bcd128_16_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(8999999999999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_bcd128_32_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(89999999999999999999999999999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_bcd128_34_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(8999999999999999999999999999999999, &mut bcd);
    });
  }
}
