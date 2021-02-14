use rtdlib::types as rtd_types;

use crate::api::aevent::EventApi;
use crate::listener::Lout;
use crate::errors::TGError;
use crate::observer;
use crate::tip;
use rtdlib::types::*;

pub struct Handler<'a> {
  api: &'a EventApi,
  lout: &'a Lout,
  warn_unregister_listener: &'a bool,
}

impl<'a> Handler<'a> {
  pub(crate) fn new(api: &'a EventApi, lout: &'a Lout, warn_unregister_listener: &'a bool) -> Self {
    Self {
      api,
      lout,
      warn_unregister_listener,
    }
  }

  pub fn handle(&self, json: &'a String) {
    if let Some(ev) = self.lout.receive() {
      if let Err(e) = ev((self.api, json)) {
        if let Some(ev) = self.lout.exception() { ev((self.api, &e)); }
      }
    }
    match rtd_types::from_json::<rtd_types::TdType>(json) {
      Ok(t) => {
        match self.lout.handle_type(self.api, &t) {
          Ok(true) => return,
          Ok(false) => {
            if *self.warn_unregister_listener {
              warn!("{}", tip::un_register_listener(stringify!(t)));
            }
          }
          Err(_err) => {
            if let Some(ev) = self.lout.exception() {
              ev((self.api, &TGError::new("EVENT_HANDLER_ERROR")));
            }
          }
        }

        // observer handler
        observer::notify(t);
      }
      Err(e) => {
        error!("{}", tip::data_fail_with_json(json));
        // eprintln!("{:?}", e);
        error!("{:?}", e);
        if let Some(ev) = self.lout.exception() { ev((self.api, &TGError::new("DESERIALIZE_JSON_FAIL"))); }
      }
    }
  }
}

