use std::borrow::Borrow;
use std::sync::Arc;

use rtdlib::errors::RTDResult;
use rtdlib::Tdlib;
use rtdlib::types::RFunction;

use crate::api::aasync::AsyncApi;
use crate::api::aevent::EventApi;

#[derive(Debug, Clone)]
pub struct ApiBuilder {
  inner: Api
}

impl ApiBuilder {
  pub fn new() -> Self {
    Self {
      inner: Api {
        tdlib: Arc::new(Tdlib::new()),
        log: true,
        unsafe_log: false,
      }
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

  fn safe_log(&self, text: &String) -> String {
    if self.unsafe_log {
      return text.clone();
    }
    if text.contains("api_id") || text.contains("api_hash") {
      let regex_api_id = regex::Regex::new(r#"api_id":\d*"#).expect("Regex fail");
      let hide_api_id = regex_api_id.replace_all(text, r#"api_id":"****""#);
      let regex_api_hash = regex::Regex::new(r#"api_hash":"[0-9|a-f]*""#).expect("Regex fail");
      let hide_api_hash = regex_api_hash.replace_all(&hide_api_id, r#"api_hash":"**********""#);
      hide_api_hash.into_owned()
    } else {
      text.clone()
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
      if receive.is_some() {
        debug!("<=== {}", receive.clone().map_or("<NONE>".to_string(), |v| self.safe_log(&v)));
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
