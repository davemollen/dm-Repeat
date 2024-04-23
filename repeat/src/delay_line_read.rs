use crate::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt, MAX_REPEATS,
};

#[derive(Clone, Copy)]
pub struct DelayLineRead {
  previous_time: f32,
}

impl DelayLineRead {
  pub fn new() -> Self {
    Self { previous_time: 0. }
  }

  pub fn process(
    &mut self,
    input: f32,
    delay_line: &mut DelayLine,
    time_in_ms: f32,
    repeats: usize,
    feedback: f32,
    skew: f32,
  ) -> f32 {
    (0..MAX_REPEATS)
      .take(repeats)
      .map(|index| {
        let index = index as f32;
        let gain = self.simulate_feedback(index, feedback, repeats);
        if index == 0. {
          input * gain
        } else {
          let time = self.get_delay_time(index, time_in_ms, skew);
          delay_line.read(time, Interpolation::Step) * gain
        }
      })
      .sum()
  }

  fn reverse_indices(&self, index: f32, input: f32, repeats: usize) -> f32 {
    if input.signum() == 1. {
      index
    } else {
      repeats as f32 - index - 1.0
    }
  }

  fn simulate_feedback(&self, index: f32, feedback: f32, repeats: usize) -> f32 {
    let feedback_index = self.reverse_indices(index, feedback, repeats);
    let absolute_feedback = feedback.abs();
    if absolute_feedback == 1. {
      1.
    } else {
      absolute_feedback.fast_pow(feedback_index)
    }
  }

  fn get_delay_time(&mut self, index: f32, time: f32, skew: f32) -> f32 {
    if index == 0. {
      self.previous_time = 0.0;
      0.0
    } else if skew == 0. {
      time * index
    } else {
      let exponential_skew = skew.abs().fast_pow(2.) * if skew < 0. { -0.5 } else { 1. } + 1.;
      let delay_time = if index == 1. {
        exponential_skew.fast_pow(index - 1.) * time
      } else {
        exponential_skew.fast_pow(index - 1.) * time + self.previous_time
      };
      self.previous_time = delay_time;
      delay_time
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::delay_line_read::DelayLineRead;

  #[test]
  fn feedback() {
    let repeater = DelayLineRead::new();
    assert_eq!(repeater.simulate_feedback(0.0, 1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(1.0, 1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(2.0, 1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(3.0, 1.0, 4), 1.0);

    assert_eq!(repeater.simulate_feedback(0.0, 2.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(1.0, 2.0, 4), 2.0);
    assert_eq!(repeater.simulate_feedback(2.0, 2.0, 4), 4.0);
    assert_eq!(repeater.simulate_feedback(3.0, 2.0, 4), 8.0);

    assert_eq!(repeater.simulate_feedback(0.0, 0.5, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(1.0, 0.5, 4), 0.5);
    assert_eq!(repeater.simulate_feedback(2.0, 0.5, 4), 0.25);
    assert_eq!(repeater.simulate_feedback(3.0, 0.5, 4), 0.125);

    assert_eq!(repeater.simulate_feedback(0.0, -1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(1.0, -1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(2.0, -1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(3.0, -1.0, 4), 1.0);

    assert_eq!(repeater.simulate_feedback(0.0, -2.0, 4), 8.0);
    assert_eq!(repeater.simulate_feedback(1.0, -2.0, 4), 4.0);
    assert_eq!(repeater.simulate_feedback(2.0, -2.0, 4), 2.0);
    assert_eq!(repeater.simulate_feedback(3.0, -2.0, 4), 1.0);

    assert_eq!(repeater.simulate_feedback(0.0, -0.5, 4), 0.125);
    assert_eq!(repeater.simulate_feedback(1.0, -0.5, 4), 0.25);
    assert_eq!(repeater.simulate_feedback(2.0, -0.5, 4), 0.5);
    assert_eq!(repeater.simulate_feedback(3.0, -0.5, 4), 1.0);
  }

  #[test]
  fn delay_time() {
    let mut repeater = DelayLineRead::new();
    assert_eq!(repeater.get_delay_time(0.0, 100.0, 0.0), 0.0);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, 0.0), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, 0.0), 200.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, 0.0), 300.0);

    assert_eq!(repeater.get_delay_time(0.0, 100.0, 1.0), 0.0);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, 1.0), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, 1.0), 300.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, 1.0), 700.0);

    assert_eq!(repeater.get_delay_time(0.0, 100.0, -1.0), 0.0);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, -1.0), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, -1.0), 150.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, -1.0), 175.0);
  }
}
