extern crate lv2;
extern crate repeat;
use lv2::prelude::*;
use repeat::Repeat;

#[derive(PortCollection)]
struct Ports {
  freq: InputPort<InPlaceControl>,
  repeats: InputPort<InPlaceControl>,
  feedback: InputPort<InPlaceControl>,
  skew: InputPort<InPlaceControl>,
  limiter: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output: OutputPort<InPlaceAudio>,
}

#[uri("https://github.com/davemollen/dm-Repeat")]
struct DmRepeat {
  repeat: Repeat,
  is_active: bool,
}

impl Plugin for DmRepeat {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      repeat: Repeat::new(_plugin_info.sample_rate() as f32),
      is_active: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let time = (ports.freq.get()).recip() * 1000.;
    let repeats = ports.repeats.get() as usize;
    let feedback = ports.feedback.get() * 0.01;
    let skew = ports.skew.get() * 0.01;
    let limiter = ports.limiter.get() == 1.;

    if !self.is_active {
      self.repeat.initialize_params(time, repeats, feedback, skew);
      self.is_active = true;
    }

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let repeat_output = self
        .repeat
        .process(input.get(), time, repeats, feedback, skew, limiter);
      output.set(repeat_output);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmRepeat);
