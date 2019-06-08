use crate::types::t_update_chat_last_message::TGUpdateChatLastMessage;
use crate::errors;
use crate::types::t_message::TGMessage;
use rtdlib::types::RObject;

impl TGUpdateChatLastMessage {

  pub fn chat_id(&self) -> i64 { self.td_origin().chat_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn last_message(&self) -> TGMessage { TGMessage::from_json(self.td_origin().last_message().expect(errors::TELEGRAM_DATA_FAIL).to_json()).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn order(&self) -> i64 { self.td_origin().order().map(|v| toolkit::number::as_i64(v).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }


}
