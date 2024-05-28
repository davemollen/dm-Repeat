mod ui;
use crate::RepeatParameters;
use std::sync::Arc;
pub use ui::plugin_gui;
use vizia::{prelude::WindowSize, Application, ParentWindow};
use vst::{editor::Editor, prelude::HostCallback};

pub const WINDOW_SIZE: WindowSize = WindowSize {
  width: 400,
  height: 200,
};

pub struct RepeatEditor {
  pub params: Arc<RepeatParameters>,
  pub is_open: bool,
  pub host: Option<HostCallback>,
}

impl Editor for RepeatEditor {
  fn position(&self) -> (i32, i32) {
    (0, 0)
  }

  fn size(&self) -> (i32, i32) {
    (WINDOW_SIZE.width as i32, WINDOW_SIZE.height as i32)
  }

  fn open(&mut self, parent: *mut ::std::ffi::c_void) -> bool {
    if self.is_open {
      return false;
    }

    self.is_open = true;

    let host = self.host;
    let params = self.params.clone();

    Application::new(move |cx| plugin_gui(cx, Arc::clone(&params), host))
      .title("dm-Repeat")
      .inner_size(WINDOW_SIZE)
      .open_parented(&ParentWindow(parent));

    true
  }

  fn is_open(&mut self) -> bool {
    self.is_open
  }

  fn close(&mut self) {
    self.is_open = false;
  }
}
