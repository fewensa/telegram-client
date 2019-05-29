use std::str::FromStr;
use std::sync::Mutex;

use slog::{Drain, FnValue, Level, Logger};

lazy_static! {
  /// Documentation!
  static ref LOGGER_EXAMPLES: Logger = log("examples");
}

fn level() -> Level {
  let level = ::std::env::var("LOG_LEVEL");
  match level {
    Ok(lv) => {
      match Level::from_str(&lv[..]) {
        Ok(lvo) => lvo,
        Err(_error) => {
          if cfg!(debug_assertions) { Level::Debug } else { Level::Warning }
        }
      }
    }
    Err(_error) => {
      if cfg!(debug_assertions) { Level::Debug } else { Level::Warning }
    }
  }
}

fn log(t: &'static str) -> Logger {
  let decorator = slog_term::TermDecorator::new().build();
  let drain = Mutex::new(slog_term::FullFormat::new(decorator).build())
    .filter_level(level())
    .fuse();
  slog::Logger::root(drain, o!(
    "version" => env!("CARGO_PKG_VERSION"),
    "type" => t,
    "location" => FnValue(move |info| {
        format!("({}:{}) [{}]", info.file(), info.line(), info.module(), )
      })
  ))
}

pub fn examples() -> &'static Logger {
  &LOGGER_EXAMPLES
}


