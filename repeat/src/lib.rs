mod delay_line;
mod delay_line_read;
mod float_ext;
mod ramp;
use {
  delay_line::DelayLine, delay_line_read::DelayLineRead, float_ext::FloatExt, ramp::Ramp, std::f32,
  std::f32::consts::FRAC_PI_2,
};

pub const MAX_REPEATS: usize = 32;

#[derive(PartialEq, Clone, Copy)]
pub struct Params {
  repeats: usize,
  time: f32,
  feedback: f32,
  skew: f32,
}

pub struct Repeat {
  delay_line: DelayLine,
  repeats: [DelayLineRead; 2],
  ramp: Ramp,
  active_index: usize,
}

impl Repeat {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new(sample_rate as usize * 10, sample_rate),
      repeats: [DelayLineRead::new(), DelayLineRead::new()],
      ramp: Ramp::new(sample_rate, 5.),
      active_index: 0,
    }
  }

  pub fn initialize_params(&mut self, time: f32, repeats: usize, feedback: f32, skew: f32) {
    self.repeats[self.active_index].initialize(time, repeats, feedback, skew)
  }

  pub fn process(
    &mut self,
    input: f32,
    time: f32,
    repeats: usize,
    feedback: f32,
    skew: f32,
  ) -> f32 {
    let repeated = self.repeat(input, time, repeats, feedback, skew);
    self.delay_line.write(input);
    repeated
  }

  fn crossfade(&mut self, input: f32) -> f32 {
    let Self {
      delay_line,
      repeats,
      ..
    } = self;

    let ramp = self.ramp.process();
    let window = (ramp * FRAC_PI_2).fast_sin();
    let window = window * window;

    let (window_a, window_b) = if self.active_index == 0 {
      (window, 1. - window)
    } else {
      (1. - window, window)
    };

    let a = repeats[0].process(input, delay_line) * window_a;
    let b = repeats[1].process(input, delay_line) * window_b;
    a + b
  }

  fn repeat(&mut self, input: f32, time: f32, repeats: usize, feedback: f32, skew: f32) -> f32 {
    let current_params = Params {
      repeats,
      time,
      feedback,
      skew,
    };

    let parameters_have_changed = current_params != self.repeats[self.active_index].get_params();

    match (parameters_have_changed, self.ramp.is_finished()) {
      (false, true) => self.repeats[self.active_index].process(input, &mut self.delay_line),
      (true, true) => {
        self.active_index = self.active_index + 1 & 1;
        self.repeats[self.active_index].initialize(time, repeats, feedback, skew);
        self.ramp.start();
        self.crossfade(input)
      }
      _ => self.crossfade(input),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Params;

  #[test]
  fn next_and_previous_parameters_equality() {
    assert!(
      Params {
        repeats: 4,
        time: 200.,
        feedback: 0.,
        skew: 0.,
      } == Params {
        repeats: 4,
        time: 200.,
        feedback: 0.,
        skew: 0.,
      }
    );

    assert!(
      Params {
        repeats: 4,
        time: 200.,
        feedback: 0.,
        skew: 0.,
      } != Params {
        repeats: 4,
        time: 1000.,
        feedback: 0.,
        skew: 0.,
      }
    );

    assert!(
      Params {
        repeats: 8,
        time: 200.,
        feedback: 0.,
        skew: 0.,
      } != Params {
        repeats: 4,
        time: 1000.,
        feedback: 0.,
        skew: 0.,
      }
    );
  }
}
