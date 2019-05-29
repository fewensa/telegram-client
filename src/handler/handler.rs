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

  pub fn handle(&self, json: &'a String, update: &'a Box<Update>) {
    match self.lout.update() {
      Some(fnc) => (*fnc)((self.api, update)),
      None => {
        match UpdateHandler::new(self.api, self.lout).handle(update) {
          Ok(_) => {}
          Err(e) => error!(tglog::telegram(), "{:?}", e)
        }
      }
    }
  }
}

