#[path = "./editor/components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
#[path = "./editor/components/param_checkbox.rs"]
mod param_checkbox;
use nih_plug::params::Param;
use param_checkbox::ParamCheckbox;
mod ui_data;
use crate::repeat_parameters::RepeatParameters;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::{
  model::Model,
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::Units::{Pixels, Stretch},
  style::FontWeightKeyword,
  views::{HStack, Label, VStack},
};
use nih_plug_vizia::{create_vizia_editor, vizia_assets, ViziaState, ViziaTheming};
use std::sync::Arc;
use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(|| (400, 200))
}

pub(crate) fn create(
  params: Arc<RepeatParameters>,
  editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(
    editor_state,
    ViziaTheming::Custom,
    move |cx, gui_context| {
      vizia_assets::register_roboto(cx);
      vizia_assets::register_roboto_bold(cx);
      cx.add_stylesheet(STYLE).ok();

      UiData {
        params: params.clone(),
        gui_context: gui_context.clone(),
      }
      .build(cx);

      VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.freq.name(),
            UiData::params,
            params.freq.as_ptr(),
            |params| &params.freq,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.repeats.name(),
            UiData::params,
            params.repeats.as_ptr(),
            |params| &params.repeats,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.feedback.name(),
            UiData::params,
            params.feedback.as_ptr(),
            |params| &params.feedback,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.skew.name(),
            UiData::params,
            params.skew.as_ptr(),
            |params| &params.skew,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          )
          .top(Pixels(12.0));

          ParamCheckbox::new(
            cx,
            params.limiter.name(),
            UiData::params,
            params.limiter.as_ptr(),
            |params| &params.limiter,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
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
    },
  )
}
