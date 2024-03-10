use std::fmt::Display;

use tracing_subscriber::fmt::SubscriberBuilder;

pub mod builder;
pub mod format;
pub mod level;
pub mod prelude;

#[allow(unused)]
pub fn session() -> SubscriberBuilder {
  tracing_subscriber::fmt()
}

#[macro_export]
macro_rules! log_lib_info {
  () => {{
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    // const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

    tracing::info!("{NAME} v{VERSION}");
  }};
}

pub trait LogErr {
  fn log_error(self) -> Self
  where
    Self: Sized;
  fn log_warn(self) -> Self
  where
    Self: Sized;
  fn log_error_msg(self, msg: &'static str) -> Self
  where
    Self: Sized;
  fn log_warn_msg(self, msg: &'static str) -> Self
  where
    Self: Sized;
}

impl<T, E: Display> LogErr for Result<T, E> {
  fn log_error(self) -> Self
  where
    Self: Sized,
  {
    if let Err(error) = &self {
      tracing::error!("{error}");
    }

    self
  }

  fn log_warn(self) -> Self
  where
    Self: Sized,
  {
    if let Err(error) = &self {
      tracing::warn!("{error}");
    }

    self
  }

  fn log_error_msg(self, msg: &'static str) -> Self
  where
    Self: Sized,
  {
    if let Err(error) = &self {
      tracing::error!("`{msg}`: {error}");
    }

    self
  }

  fn log_warn_msg(self, msg: &'static str) -> Self
  where
    Self: Sized,
  {
    if let Err(error) = &self {
      tracing::warn!("`{msg}`: {error}");
    }

    self
  }
}
