use rtdlib::types as td_types;
use rtdlib::types::MessageForwardOrigin;

use crate::errors;
use crate::types::t_forward::*;

impl TGMessageForwardInfo {
  pub fn origin(&self) -> TGMessageForwardOrigin { self.td_origin().origin().map(|v| TGMessageForwardOrigin::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn date(&self) -> i32 { self.td_origin().date().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn from_chat_id(&self) -> i64 { self.td_origin().from_chat_id().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn from_message_id(&self) -> i64 { self.td_origin().from_message_id().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }


  pub fn is_channel(&self) -> bool { self.origin().is_channel() }
  pub fn is_hidden_user(&self) -> bool { self.origin().is_hidden_user() }
  pub fn is_user(&self) -> bool { self.origin().is_user() }
}


#[derive(Debug, Clone)]
pub enum TGMessageForwardOrigin {
  Channel(TGMessageForwardOriginChannel),
  HiddenUser(TGMessageForwardOriginHiddenUser),
  User(TGMessageForwardOriginUser),
}


impl TGMessageForwardOrigin {
  fn of(td: Box<td_types::MessageForwardOrigin>) -> Self {
    tuple_rtd_type_mapping!(
      MessageForwardOrigin,
      TGMessageForwardOrigin,
      RTDMessageForwardOriginType,
      (MessageForwardOriginChannel     , Channel        ,  TGMessageForwardOriginChannel       );
      (MessageForwardOriginHiddenUser  , HiddenUser     ,  TGMessageForwardOriginHiddenUser    );
      (MessageForwardOriginUser        , User           ,  TGMessageForwardOriginUser          );
    )(td)
  }

  pub fn is_channel(&self) -> bool { tuple_enum_is!(TGMessageForwardOrigin, Channel     )(self) }
  pub fn is_hidden_user(&self) -> bool { tuple_enum_is!(TGMessageForwardOrigin, HiddenUser  )(self) }
  pub fn is_user(&self) -> bool { tuple_enum_is!(TGMessageForwardOrigin, User        )(self) }

  pub fn on_channel<F: FnOnce(&TGMessageForwardOriginChannel)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageForwardOrigin, Channel    , |t| fnc(t))(self);
    self
  }
  pub fn on_hidden_user<F: FnOnce(&TGMessageForwardOriginHiddenUser)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageForwardOrigin, HiddenUser , |t| fnc(t))(self);
    self
  }
  pub fn on_user<F: FnOnce(&TGMessageForwardOriginUser)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageForwardOrigin, User       , |t| fnc(t))(self);
    self
  }
}


impl TGMessageForwardOriginChannel {
  pub fn chat_id(&self) -> i64 { self.td_origin().chat_id().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn message_id(&self) -> i64 { self.td_origin().message_id().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn author_signature(&self) -> Option<String> { self.td_origin().author_signature() }
}

impl TGMessageForwardOriginHiddenUser {
  pub fn sender_name(&self) -> String { self.td_origin().sender_name().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessageForwardOriginUser {
  pub fn sender_user_id(&self) -> i32 { self.td_origin().sender_user_id().expect(errors::TELEGRAM_DATA_FAIL) }
}






