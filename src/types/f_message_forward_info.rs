use rtdlib::types as td_types;

use crate::errors;
use crate::types::t_message_forward_info::TGMessageForwardInfo;

impl TGMessageForwardInfo {

  pub fn forward_origin(&self) -> Option<Box<td_types::MessageForwardOrigin>> { self.origin.clone() }

  pub fn date(&self) -> i32 { self.origin().date().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn from_chat_id(&self) -> i64 { self.origin().from_chat_id().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn from_message_id(&self) -> i64 { self.origin().from_message_id().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }

}
