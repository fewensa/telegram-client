use crate::errors;
use crate::types::t_chat_type::*;

impl TGChatTypeBasicGroup {
  pub fn basic_group_id(&self) -> i32 { self.origin().basic_group_id().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGChatTypePrivate {
  pub fn user_id(&self) -> i32 { self.origin().user_id().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGChatTypeSecret {
  pub fn secret_chat_id(&self) -> i32 { self.origin().secret_chat_id().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn user_id(&self) -> i32 { self.origin().user_id().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGChatTypeSupergroup {
  pub fn supergroup_id(&self) -> i32 { self.origin().supergroup_id().map(|v| v).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn is_channel(&self) -> bool { self.origin().is_channel().map_or(false, |v| v) }
}
