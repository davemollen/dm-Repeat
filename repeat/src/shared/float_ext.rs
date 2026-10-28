pub trait FloatExt {
  fn mix(self, right: Self, factor: Self) -> Self;
  fn cubic_spline_curve(self) -> Self;
  fn fast_sin(self) -> Self;
  fn fast_pow(self, exponent: Self) -> Self;
  fn mstosamps(self, sample_rate: Self) -> Self;
}

impl FloatExt for f32 {
  #[inline(always)]
  fn mix(self, right: Self, factor: Self) -> Self {
    self + (right - self) * factor
  }

  #[inline(always)]
  fn cubic_spline_curve(self) -> Self {
    self * self * (3.0 - 2.0 * self)
  }

  /// This is a sine approximation. Use this to safe processing power.
  #[inline(always)]
  fn fast_sin(self) -> Self {
    const TWOPI: f32 = 6.2831853071795865;
    const INVTWOPI: f32 = 0.15915494309189534;
    let k: u32 = (self * INVTWOPI) as u32;
    let half = if self < 0_f32 { -0.5_f32 } else { 0.5_f32 };
    let x = (half + (k as f32)) * TWOPI - self;
    sin_approx(x)
  }

  #[inline(always)]
  fn fast_pow(self, exponent: Self) -> Self {
    pow2(exponent * log2(self))
  }

  /// Convert milliseconds to samples based on the samplerate.
  #[inline(always)]
  fn mstosamps(self, sample_rate: Self) -> Self {
    self * 0.001 * sample_rate
  }
}

#[cfg(test)]
mod tests {
  use std::f32::consts::PI;

  use crate::assert_approximately_eq;

  use super::FloatExt;

  #[test]
  fn mix() {
    assert_eq!((1f32).mix(0., 0.), 1.);
    assert_eq!((1f32).mix(0., 0.5), 0.5);
    assert_eq!((1f32).mix(0., 1.), 0.);
  }

  #[test]
  fn fast_sin() {
    assert_approximately_eq!((0.1).fast_sin(), (0.1f32).sin(), 3);
    assert_approximately_eq!((PI).fast_sin(), (PI).sin(), 3);
    assert_approximately_eq!((PI * 1.5).fast_sin(), (PI * 1.5).sin(), 3);
    assert_approximately_eq!((PI * -1.9).fast_sin(), (PI * -1.9).sin(), 3);
  }
}

fn sin_approx(x: f32) -> f32 {
  const FOUROVERPI: f32 = 1.2732395447351627;
  const FOUROVERPISQ: f32 = 0.40528473456935109;
  const Q: f32 = 0.77633023248007499;

  let mut p = 0.22308510060189463_f32.to_bits();
  let mut v = x.to_bits();

  let sign: u32 = v & 0x80000000;
  v &= 0x7FFFFFFF;

  let qpprox = FOUROVERPI * x - FOUROVERPISQ * x * f32::from_bits(v);

  p |= sign;

  qpprox * (Q + f32::from_bits(p) * qpprox)
}

fn log2(x: f32) -> f32 {
  let mut y = x.to_bits() as f32;
  y *= 1.1920928955078125e-7_f32;
  y - 126.94269504_f32
}

fn pow2(p: f32) -> f32 {
  let clipp = if p < -126.0 { -126.0_f32 } else { p };
  let v = ((1 << 23) as f32 * (clipp + 126.94269504_f32)) as u32;
  f32::from_bits(v)
}
