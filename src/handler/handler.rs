use rtdlib::types::*;

use crate::api::Api;
use crate::handler::handler_update::UpdateHandler;
use crate::listener::{Listener, Lout};
use crate::tglog;

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
      error!(tglog::telegram(), "Json fail, can not covert to Box<Object>    {:?}", json);
      return;
    }
    let object = object.unwrap();

//    if let Some(fnc) = self.lout.update() {
//      (*fnc)((self.api, &update));
//    }
    if let Err(e) = UpdateHandler::new(self.api, self.lout).handle(&object) {
      error!(tglog::telegram(), "{:?}", e);
    }
  }
}

