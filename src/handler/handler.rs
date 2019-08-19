use rtdlib::types::*;

use crate::api::Api;
use crate::handler::handler_receive::ReceiveHandler;
use crate::listener::{Listener, Lout};
use crate::tglog;
use crate::errors;

pub struct Handler<'a> {
  api: &'a Api,
  lout: &'a Lout,
}

impl<'a> Handler<'a> {
  pub(crate) fn new(api: &'a Api, lout: &'a Lout) -> Self {
    Self {
      api,
      lout,
    }
  }

  pub fn handle(&self, json: &'a String) {
    let object: Option<Box<Object>> = Object::from_json(json);
    if object.is_none() {
      error!(tglog::telegram(), "{}", &errors::data_fail_with_json(json)[..]);
      return;
    }
    let object = object.unwrap();

    if !self.lout.is_support(object.td_name()) {
      warn!(tglog::telegram(), "{}", &errors::not_have_listener(object.td_name())[..]);
    }

    if let Some(fnc) = self.lout.receive() {
      (*fnc)((self.api, &object));
    }
    if let Err(e) = ReceiveHandler::new(self.api, self.lout).handle(&object) {
      error!(tglog::telegram(), "\n{:?}", e);
    }
  }
}

