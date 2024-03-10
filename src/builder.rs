use tracing_subscriber::{
  fmt::{
    format::{DefaultFields, Format},
    SubscriberBuilder,
  },
  EnvFilter,
};

pub struct LoggingSession {
  filter: EnvFilter,
  thread_names: bool,
  file_names: bool,
  line_numbers: bool,
}

impl Default for LoggingSession {
  fn default() -> Self {
    Self {
      filter: "".into(),
      thread_names: false,
      file_names: false,
      line_numbers: false,
    }
  }
}

impl LoggingSession {
  pub fn with_filter(mut self, filter: impl Into<EnvFilter>) -> Self {
    self.filter = filter.into();
    self
  }

  pub fn with_thread_names(mut self, enable: bool) -> Self {
    self.thread_names = enable;
    self
  }

  pub fn with_file_names(mut self, enable: bool) -> Self {
    self.file_names = enable;
    self
  }

  pub fn with_line_numbers(mut self, enable: bool) -> Self {
    self.line_numbers = enable;
    self
  }

  pub fn finalize(self) -> SubscriberBuilder<DefaultFields, Format, EnvFilter> {
    tracing_subscriber::fmt()
      .with_env_filter(self.filter)
      .with_thread_names(self.thread_names)
      .with_file(self.file_names)
      .with_line_number(self.line_numbers)
  }

  pub fn start(self) {
    self.finalize().init();
  }
}

#[macro_export]
macro_rules! logging_session {
  () => {{
    const NAME: &str = env!("CARGO_PKG_NAME");
    $crate::builder::LoggingSession::default().with_filter($crate::format::format_filter_slice(&[
      ("RUST_LOG", None),
      (NAME, Some($crate::level::LogLevel::Trace)),
    ]))
  }};
}

#[macro_export]
macro_rules! logging_session_ex {
    ($($levels:expr),+) => {{
        const NAME: &str = env!("CARGO_PKG_NAME");
        $crate::builder::LoggingSession::default()
            .with_filter($crate::format::format_filter_slice(&[("RUST_LOG", None), (NAME, Some($crate::level::LogLevel::Trace)), $($levels),+]))
    }};
}

#[macro_export]
macro_rules! debug_logging_session {
  () => {{
    if cfg!(debug_assertions) {
      Some($crate::logging_session!())
    } else {
      None
    }
  }};
}

#[macro_export]
macro_rules! debug_logging_session_ex {
  ($($levels:expr),+) => {{
    if cfg!(debug_assertions) {
      Some($crate::logging_session_ex!($($levels),+))
    } else {
      None
    }
  }};
}

#[macro_export]
macro_rules! start_debug_logging_session {
  () => {{
    #[cfg(debug_assertions)]
    $crate::logging_session!().start();
  }};
}

#[macro_export]
macro_rules! start_debug_logging_session_ex {
  ($($levels:expr),+) => {{
    #[cfg(debug_assertions)]
    $crate::logging_session_ex!($($levels),+).start();
  }};
}
