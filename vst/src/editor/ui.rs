#[path = "./components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
#[path = "./components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
#[path = "./components/param_int_knob.rs"]
mod param_int_knob;
use param_int_knob::{ParamIntKnob, ParamIntKnobSize};
#[path = "ui_data.rs"]
mod ui_data;
use crate::repeat_parameters::RepeatParameters;
use std::sync::Arc;
use ui_data::{ParamChangeEvent, UiData};
use vizia::{
  context::Context,
  model::Model,
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::{
    FontWeightKeyword,
    Units::{Pixels, Stretch},
  },
  views::{HStack, Label, VStack},
};
use vst::prelude::HostCallback;

const STYLE: &str = include_str!("style.css");

pub fn plugin_gui(cx: &mut Context, params: Arc<RepeatParameters>, host: Option<HostCallback>) {
  let _ = cx.add_stylesheet(STYLE);

  UiData {
    params: params.clone(),
    host,
  }
  .build(cx);

  VStack::new(cx, |cx| {
    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        params.freq.name,
        UiData::params,
        |params| &params.freq,
        |val| ParamChangeEvent::SetFreq(val),
        ParamKnobSize::Regular,
      );

      ParamIntKnob::new(
        cx,
        params.repeats.name,
        UiData::params,
        |params| &params.repeats,
        |val| ParamChangeEvent::SetRepeats(val),
        ParamIntKnobSize::Regular,
      );

      ParamKnob::new(
        cx,
        params.feedback.name,
        UiData::params,
        |params| &params.feedback,
        |val| ParamChangeEvent::SetFeedback(val),
        ParamKnobSize::Regular,
      );

      ParamKnob::new(
        cx,
        params.skew.name,
        UiData::params,
        |params| &params.skew,
        |val| ParamChangeEvent::SetSkew(val),
        ParamKnobSize::Regular,
      )
      .top(Pixels(12.0));

      ParamCheckbox::new(
        cx,
        params.limiter.name,
        UiData::params,
        |params| &params.limiter,
        |val| ParamChangeEvent::SetLimiter(val),
      )
      .top(Pixels(12.0));
    })
    .child_space(Stretch(1.0))
    .col_between(Pixels(8.0));

    Label::new(cx, "dm-Repeat")
      .font_size(22.0)
      .font_weight(FontWeightKeyword::Bold)
      .border_radius(Pixels(16.0))
      .border_width(Pixels(1.))
      .border_color("#005254")
      .background_color("#009092")
      .child_space(Stretch(1.0))
      .child_top(Pixels(1.0))
      .child_bottom(Pixels(5.0))
      .width(Pixels(144.0))
      .left(Stretch(1.0));
  })
  .child_space(Pixels(16.0))
  .background_color("#161616");
}
