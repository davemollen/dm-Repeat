extern crate lv2;
extern crate repeat;
use lv2::prelude::*;
use repeat::Repeat;

#[derive(PortCollection)]
struct Ports {
  freq: InputPort<Control>,
  repeats: InputPort<Control>,
  feedback: InputPort<Control>,
  skew: InputPort<Control>,
  limiter: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
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
    let time = (*ports.freq).recip() * 1000.;
    let repeats = *ports.repeats as usize;
    let feedback = *ports.feedback * 0.01;
    let skew = *ports.skew * 0.01;
    let limiter = *ports.limiter == 1.;

    if !self.is_active {
      self.repeat.initialize_params(time, repeats, feedback, skew);
      self.is_active = true;
    }

    for (in_frame, out_frame) in ports.input.iter().zip(ports.output.iter_mut()) {
      *out_frame = self
        .repeat
        .process(*in_frame, time, repeats, feedback, skew, limiter);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmRepeat);
