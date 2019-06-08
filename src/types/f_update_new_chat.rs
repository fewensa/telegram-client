use rtdlib::types::RObject;

use crate::errors;
use crate::types::t_update_new_chat::TGUpdateNewChat;
use crate::types::TGChat;

impl TGUpdateNewChat {
  pub fn chat(&self) -> TGChat { self.td_origin().chat().map(|v| TGChat::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}
