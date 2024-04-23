use nih_plug::prelude::*;
use repeat::Repeat;
use std::sync::Arc;
mod repeat_parameters;
use repeat_parameters::RepeatParameters;
mod editor;

struct DmRepeat {
  params: Arc<RepeatParameters>,
  repeat: Repeat,
}

impl Default for DmRepeat {
  fn default() -> Self {
    let params = Arc::new(RepeatParameters::default());
    Self {
      params: params.clone(),
      repeat: Repeat::new(44100.),
    }
  }
}

impl Plugin for DmRepeat {
  const NAME: &'static str = "dm-Repeat";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-Repeat";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(1),
    main_output_channels: NonZeroU32::new(1),
    ..AudioIOLayout::const_default()
  }];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.repeat = Repeat::new(buffer_config.sample_rate);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let freq = self.params.freq.value();
    let repeats = self.params.repeats.value();
    let feedback = self.params.feedback.value();
    let skew = self.params.skew.value();

    buffer.iter_samples().for_each(|mut channel_samples| {
      let input = channel_samples.get_mut(0).unwrap();
      let repeat_output = self
        .repeat
        .process(*input, freq, repeats as usize, feedback, skew);
      let output = channel_samples.get_mut(0).unwrap();
      *output = repeat_output;
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmRepeat {
  const CLAP_ID: &'static str = "dm-Repeat";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A delay plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Stereo,
    ClapFeature::Mono,
    ClapFeature::Utility,
  ];
}

impl Vst3Plugin for DmRepeat {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-Repeat.......";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
    &[Vst3SubCategory::Fx, Vst3SubCategory::Delay];
}

nih_export_clap!(DmRepeat);
nih_export_vst3!(DmRepeat);
