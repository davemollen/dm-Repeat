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
struct VariableParameters {
  repeats: usize,
  time_in_ms: f32,
  feedback: f32,
  skew: f32,
}

pub struct Repeat {
  delay_line: DelayLine,
  repeats: [DelayLineRead; 2],
  variable_parameters: [VariableParameters; 2],
  ramp: Ramp,
}

impl Repeat {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new(sample_rate as usize * 10, sample_rate),
      repeats: [DelayLineRead::new(); 2],
      variable_parameters: [VariableParameters {
        repeats: 4,
        time_in_ms: 200.,
        feedback: 0.,
        skew: 0.,
      }; 2],
      ramp: Ramp::new(sample_rate, 5.),
    }
  }

  pub fn process(
    &mut self,
    input: f32,
    freq: f32,
    repeats: usize,
    feedback: f32,
    skew: f32,
  ) -> f32 {
    let repeated = self.repeat(input, freq, repeats, feedback, skew);
    self.delay_line.write(input);
    repeated
  }

  fn crossfade(&mut self, input: f32) -> f32 {
    let Self {
      delay_line,
      repeats,
      variable_parameters,
      ..
    } = self;

    let ramp = self.ramp.process();
    let window = (ramp * FRAC_PI_2).fast_cos();
    let window = window * window;

    let a = repeats[0].process(
      input,
      delay_line,
      variable_parameters[0].time_in_ms,
      variable_parameters[0].repeats,
      variable_parameters[0].feedback,
      variable_parameters[0].skew,
    ) * window;
    let b = repeats[1].process(
      input,
      delay_line,
      variable_parameters[1].time_in_ms,
      variable_parameters[1].repeats,
      variable_parameters[1].feedback,
      variable_parameters[1].skew,
    ) * (1. - window);
    a + b

    // (0..2)
    //   .map(|index| {
    //     let window = if index == 1 { 1. - window } else { window };
    //     repeats[index].process(
    //       input,
    //       delay_line,
    //       variable_parameters[index].time_in_ms,
    //       variable_parameters[index].repeats,
    //       variable_parameters[index].feedback,
    //       variable_parameters[index].skew,
    //     ) * window
    //   })
    //   .sum()
  }

  fn repeat(&mut self, input: f32, freq: f32, repeats: usize, feedback: f32, skew: f32) -> f32 {
    let time_in_ms = 1000. / freq;

    let current_parameters = VariableParameters {
      repeats,
      time_in_ms,
      feedback,
      skew,
    };

    let parameters_have_changed = current_parameters != self.variable_parameters[1];
    match (parameters_have_changed, self.ramp.is_finished()) {
      (false, true) => {
        let repeats_out = self.repeats[0].process(
          input,
          &mut self.delay_line,
          current_parameters.time_in_ms,
          current_parameters.repeats,
          current_parameters.feedback,
          current_parameters.skew,
        );
        self.variable_parameters[1] = current_parameters;
        repeats_out
      }
      (true, true) => {
        self.variable_parameters[0] = self.variable_parameters[1];
        self.variable_parameters[1] = current_parameters;
        // self.repeats.set_values();
        self.ramp.start();
        self.crossfade(input)
      }
      _ => self.crossfade(input),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::VariableParameters;

  #[test]
  fn next_and_previous_parameters_equality() {
    assert!(
      VariableParameters {
        repeats: 4,
        time_in_ms: 200.,
        feedback: 0.,
        skew: 0.,
      } == VariableParameters {
        repeats: 4,
        time_in_ms: 200.,
        feedback: 0.,
        skew: 0.,
      }
    );

    assert!(
      VariableParameters {
        repeats: 4,
        time_in_ms: 200.,
        feedback: 0.,
        skew: 0.,
      } != VariableParameters {
        repeats: 4,
        time_in_ms: 1000.,
        feedback: 0.,
        skew: 0.,
      }
    );

    assert!(
      VariableParameters {
        repeats: 8,
        time_in_ms: 200.,
        feedback: 0.,
        skew: 0.,
      } != VariableParameters {
        repeats: 4,
        time_in_ms: 1000.,
        feedback: 0.,
        skew: 0.,
      }
    );
  }
}
