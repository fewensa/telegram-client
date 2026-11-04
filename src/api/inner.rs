use std::borrow::Borrow;
use std::sync::{Arc, LazyLock};

use log::{debug, info};
use rtdlib::errors::RTDResult;
use rtdlib::types::RFunction;
use rtdlib::Tdlib;

use crate::api::aasync::AsyncApi;
use crate::api::aevent::EventApi;

static API_ID_RE: LazyLock<regex::Regex> =
  LazyLock::new(|| regex::Regex::new(r#""api_id"\s*:\s*"?[0-9]+"?"#).expect("api_id regex"));

static API_HASH_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
  regex::Regex::new(r#""api_hash"\s*:\s*"[0-9A-Fa-f]*""#).expect("api_hash regex")
});

/// Hide Telegram application credentials before they reach the log.
pub(crate) fn redact_secrets(text: &str) -> String {
  let hide_api_id = API_ID_RE.replace_all(text, r#""api_id":"****""#);
  API_HASH_RE
    .replace_all(&hide_api_id, r#""api_hash":"**********""#)
    .into_owned()
}

#[derive(Debug, Clone)]
pub struct ApiBuilder {
  inner: Api,
}

impl Default for ApiBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl ApiBuilder {
  pub fn new() -> Self {
    Self {
      inner: Api {
        tdlib: Arc::new(Tdlib::new()),
        log: true,
        unsafe_log: false,
      },
    }
  }

  pub fn build(&self) -> Api {
    self.inner.clone()
  }

  fn tdlib(&mut self, tdlib: Tdlib) -> &mut Self {
    self.inner.tdlib = Arc::new(tdlib);
    self
  }

  pub fn log(&mut self, open: bool) -> &mut Self {
    self.inner.log = open;
    self
  }

  pub fn unsafe_log(&mut self, unsafe_log: bool) -> &mut Self {
    self.inner.unsafe_log = unsafe_log;
    self
  }
}

#[derive(Debug, Clone)]
pub struct Api {
  tdlib: Arc<Tdlib>,
  log: bool,
  unsafe_log: bool,
}

impl Default for Api {
  fn default() -> Self {
    ApiBuilder::new().build()
  }
}

impl Api {
  pub fn builder() -> ApiBuilder {
    ApiBuilder::new()
  }

  pub fn new(tdlib: Tdlib) -> Self {
    ApiBuilder::new().tdlib(tdlib).build()
  }

  pub fn event() -> EventApi {
    Api::event_with_tdlib(Tdlib::new())
  }

  pub fn event_with_tdlib(tdlib: Tdlib) -> EventApi {
    Api::event_with_api(Api::new(tdlib))
  }

  pub fn event_with_api(api: Api) -> EventApi {
    EventApi::new(api)
  }

  pub fn rasync() -> AsyncApi {
    Api::rasync_with_tdlib(Tdlib::new())
  }

  pub fn rasync_with_tdlib(tdlib: Tdlib) -> AsyncApi {
    Api::rasync_with_api(Api::new(tdlib))
  }

  pub fn rasync_with_api(api: Api) -> AsyncApi {
    AsyncApi::new(api)
  }

  pub fn event_api(&self) -> EventApi {
    Api::event_with_api(self.clone())
  }

  pub fn rasync_api(&self) -> AsyncApi {
    Api::rasync_with_api(self.clone())
  }

  #[doc(hidden)]
  pub fn tdlib(&self) -> &Tdlib {
    self.tdlib.borrow()
  }

  fn safe_log(&self, text: &str) -> String {
    if self.unsafe_log || (!text.contains("api_id") && !text.contains("api_hash")) {
      text.to_string()
    } else {
      redact_secrets(text)
    }
  }

  pub fn send<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<()> {
    let json = fnc.to_json()?;
    if self.log {
      debug!("===> {}", self.safe_log(&json));
    }
    self.tdlib.send(&json[..]);
    Ok(())
  }

  pub fn receive(&self, timeout: f64) -> Option<String> {
    let receive = self.tdlib.receive(timeout);
    if self.log {
      if let Some(value) = receive.as_ref() {
        debug!("<=== {}", self.safe_log(value));
      }
    }
    receive
  }

  pub fn execute<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<Option<String>> {
    let json = fnc.to_json()?;
    if self.log {
      info!("===>>> {}", self.safe_log(&json));
    }
    Ok(self.tdlib.execute(&json[..]))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn safe_log_redacts_tdlib_credentials() {
    let api = Api::default();
    let raw =
      r#"{"@type":"setTdlibParameters","parameters":{"api_id":12345,"api_hash":"0123abcdef"}}"#;
    let out = api.safe_log(raw);
    assert!(!out.contains("12345"));
    assert!(!out.contains("0123abcdef"));
    assert!(out.contains(r#""api_id":"****""#));
    assert!(out.contains(r#""api_hash":"**********""#));
  }

  #[test]
  fn unsafe_log_keeps_credentials() {
    let api = Api::builder().unsafe_log(true).build();
    let raw = r#"{"api_id":12345,"api_hash":"abcdef"}"#;
    assert_eq!(api.safe_log(raw), raw);
  }

  #[test]
  fn safe_log_leaves_unrelated_payloads_unchanged() {
    let api = Api::default();
    let raw = r#"{"@type":"getMe"}"#;
    assert_eq!(api.safe_log(raw), raw);
  }
}
