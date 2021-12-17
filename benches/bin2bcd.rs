#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
  use bcd::bin2bcd;
  use test::Bencher;

  #[bench]
  fn bench_1_digit(b: &mut Bencher) {
    let mut bcd = [0; 34];
    b.iter(|| {
      bin2bcd(1, &mut bcd);
    });
  }

  #[bench]
  fn bench_8_digits(b: &mut Bencher) {
    let mut bcd = [0; 34];
    b.iter(|| {
      bin2bcd(89999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_16_digits(b: &mut Bencher) {
    let mut bcd = [0; 34];
    b.iter(|| {
      bin2bcd(8999999999999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_32_digits(b: &mut Bencher) {
    let mut bcd = [0; 34];
    b.iter(|| {
      bin2bcd(89999999999999999999999999999999, &mut bcd);
    });
  }

  #[bench]
  fn bench_34_digits(b: &mut Bencher) {
    let mut bcd = [0; 34];
    b.iter(|| {
      bin2bcd(8999999999999999999999999999999999, &mut bcd);
    });
  }
}
