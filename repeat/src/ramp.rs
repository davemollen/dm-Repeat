pub struct Ramp {
  x: f32,
  trigger: bool,
  is_active: bool,
  step_size: f32,
}

impl Ramp {
  pub fn new(sample_rate: f32, freq: f32) -> Self {
    Self {
      x: 0.,
      trigger: false,
      is_active: false,
      step_size: sample_rate.recip() * freq,
    }
  }

  pub fn reset(&mut self) {
    self.x = 0.;
    self.trigger = false;
    self.is_active = false;
  }

  pub fn start(&mut self) {
    self.trigger = true;
    self.is_active = true;
  }

  pub fn process(&mut self) -> f32 {
    if self.trigger {
      self.x = 0.;
      self.trigger = false;
    } else if self.is_active {
      self.x += self.step_size;
      if self.x >= 1. {
        self.x = 1.;
        self.is_active = false;
      }
    }

    self.x
  }

  pub fn is_finished(&self) -> bool {
    !self.is_active
  }
}

#[cfg(test)]
mod tests {
  use super::Ramp;
  use crate::assert_approximately_eq;

  #[test]
  fn forward_ramp() {
    let mut ramp = Ramp::new(10., 1.);
    ramp.start();
    assert_approximately_eq!(ramp.process(), 0., 6);
    assert_approximately_eq!(ramp.process(), 0.1, 6);
    assert_approximately_eq!(ramp.process(), 0.2, 6);
    assert_approximately_eq!(ramp.process(), 0.3, 6);
    assert_approximately_eq!(ramp.process(), 0.4, 6);
    assert_approximately_eq!(ramp.process(), 0.5, 6);
    assert_approximately_eq!(ramp.process(), 0.6, 6);
    assert_approximately_eq!(ramp.process(), 0.7, 6);
    assert_approximately_eq!(ramp.process(), 0.8, 6);
    assert_approximately_eq!(ramp.process(), 0.9, 6);
    assert_approximately_eq!(ramp.process(), 1., 6);
    assert_approximately_eq!(ramp.process(), 1., 6);
  }

  #[test]
  fn is_finished() {
    let mut ramp = Ramp::new(10., 1.);

    // forwards
    ramp.start();
    for _ in 0..11 {
      assert!(!ramp.is_finished());
      ramp.process();
    }
    assert!(ramp.is_finished());
  }
}
