#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
  use bcd::{bin2bcd128, BCD_DIGITS_128};
  use test::Bencher;

  #[bench]
  fn bench_bcd2bin8_1_digit(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(1, &mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin8_2_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(89, &mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin8_3_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(255, &mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin128_1_digit(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(1, &mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin128_8_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(89999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin128_16_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(8999999999999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin128_32_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(89999999999999999999999999999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_bcd2bin128_34_digits(b: &mut Bencher) {
    let mut bcd = [0; BCD_DIGITS_128];
    b.iter(|| {
      bin2bcd128(8999999999999999999999999999999999, &mut bcd);
    });
  }
}
