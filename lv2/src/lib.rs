extern crate lv2;
extern crate repeat;
use lv2::prelude::*;
use repeat::{Bypass, Repeat};

#[derive(PortCollection)]
struct Ports {
  freq: InputPort<InPlaceControl>,
  repeats: InputPort<InPlaceControl>,
  feedback: InputPort<InPlaceControl>,
  skew: InputPort<InPlaceControl>,
  limiter: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output: OutputPort<InPlaceAudio>,
  enabled: InputPort<InPlaceControl>,
}

#[uri("https://github.com/davemollen/dm-Repeat")]
struct DmRepeat {
  repeat: Repeat,
  bypass: Bypass,
  is_initialized: bool,
}

impl Plugin for DmRepeat {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      repeat: Repeat::new(sample_rate),
      bypass: Bypass::new(sample_rate),
      is_initialized: false,
    })
  }

  /// Reset the internal state of the plugin.
  fn activate(&mut self, _features: &mut Self::InitFeatures) {
    self.repeat.reset();
    self.bypass.reset();
    self.is_initialized = false;
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), sample_count: u32) {
    self.bypass.set_enabled(ports.enabled.get() == 0., || {
      self.repeat.reset();
      self.is_initialized = false;
    });

    if self.bypass.is_done_activating() {
      ports.output.copy_from(&ports.input, sample_count);
      return;
    }

    let time = (ports.freq.get()).recip() * 1000.;
    let repeats = ports.repeats.get() as usize;
    let feedback = ports.feedback.get() * 0.01;
    let skew = ports.skew.get() * 0.01;
    let limiter = ports.limiter.get() == 1.;

    if !self.is_initialized {
      self.repeat.initialize_params(time, repeats, feedback, skew);
      self.is_initialized = true;
    }

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let dry = input.get();
      let wet = self
        .repeat
        .process(dry, time, repeats, feedback, skew, limiter);
      let repeat_output = self.bypass.process(dry, wet);
      output.set(repeat_output);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmRepeat);
