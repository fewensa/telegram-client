use rtdlib::types::RObject;

use crate::errors;
use crate::types::t_message::TGMessage;
use crate::types::t_update_new_message::TGUpdateNewMessage;

impl TGUpdateNewMessage {
  pub fn message(&self) -> TGMessage { TGMessage::from_json(self.origin().message().expect(errors::TELEGRAM_DATA_FAIL).to_json()).expect(errors::TELEGRAM_DATA_FAIL) }
}

