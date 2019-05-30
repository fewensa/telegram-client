use error_chain_mini::ErrorKind;
use rtdlib::types::*;

use crate::api::Api;
use crate::errors::{TGError, TGErrorKind, TGResult};
use crate::handler::*;
use crate::listener::Lout;
use crate::tglog;

pub struct UpdateHandler<'a> {
  api: &'a Api,
  lout: &'a Lout,
}

impl<'a> UpdateHandler<'a> {
  pub fn new(api: &'a Api, lout: &'a Lout) -> Self {
    Self { api, lout }
  }

  pub fn handle(&self, object: &'a Box<Object>) -> TGResult<()> {
    let td_type = object.td_type();
//    debug!(tglog::telegram(), "Update td type: {:?}", td_type);

    let tdupdatetype = RTDUpdateType::of(object.td_name());
    if tdupdatetype.is_none() {
      return Ok(());
    }
    let json = object.to_json();
    let tdupdatetype = tdupdatetype.unwrap();

    /// auto generate trait handler,
    /// # Fields
    /// - `$name` Current update tdtype struct
    /// - `$trait_fname` this struct sub trait function name
    /// - `$td_type` sub trait tdtype enum name
    /// - `$type_item` sub trait tdtype enum item name
    /// - `$listen_fname` sub trait tdtype => listen function name
    macro_rules! handler_trait {
      ($name:ident, $trait_fname:ident, $td_type:ident, ($(($type_item:ident, $listen_fname:ident));*;)) => {{
        let update = $name::from_json(json).ok_or(TGErrorKind::RTDLibFromError.into_err())?;
        match update.$trait_fname() {
          Some(t) => match $td_type::of(t.td_name()) {
            $(
              Some($td_type::$type_item) => {
                if let Some(fnc) = self.lout.$listen_fname() {
                  (*fnc)((self.api, &rtdlib::types::$type_item::from_json(t.to_json()).unwrap()))
                }
              }
            )*
            None => {}
          }
          None => {}
        }
      }}
    }

    let r = match tdupdatetype {
      RTDUpdateType::UpdateOption => {
        handler_trait!(UpdateOption, value, RTDOptionValueType, (
          (OptionValueBoolean, option_bool);
          (OptionValueEmpty, option_empty);
          (OptionValueInteger, option_integer);
          (OptionValueString, option_string);
        ))
      }
      RTDUpdateType::UpdateAuthorizationState => {
        handler_trait!(UpdateAuthorizationState, authorization_state, RTDAuthorizationStateType, (
          (AuthorizationStateClosed, authorization_state_closed);
          (AuthorizationStateClosing, authorization_state_closing);
          (AuthorizationStateLoggingOut, authorization_state_logging_out);
          (AuthorizationStateReady, authorization_state_ready);
          (AuthorizationStateWaitCode, authorization_state_wait_code);
          (AuthorizationStateWaitEncryptionKey, authorization_state_wait_encryption_key);
          (AuthorizationStateWaitPassword, authorization_state_wait_password);
          (AuthorizationStateWaitPhoneNumber, authorization_state_wait_phone_number);
          (AuthorizationStateWaitTdlibParameters, authorization_state_wait_tdlibparameters);
        ))
      }
      _ => ()
    };
    Ok(r)
  }
}




