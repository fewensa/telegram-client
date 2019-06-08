
use crate::types::TGUpdateDeleteMessages;
use crate::errors;

impl TGUpdateDeleteMessages {

  // {"@type":"updateDeleteMessages","chat_id":690763082,"message_ids":[145752064],"is_permanent":true,"from_cache":false}

  pub fn chat_id(&self) -> i64 { self.td_origin().chat_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn message_ids(&self) -> Vec<i64> { self.td_origin().message_ids().map_or(vec![], |v| v) }

  pub fn is_permanent(&self) -> bool { self.td_origin().is_permanent().map_or(false, |v| v) }

  pub fn from_cache(&self) -> bool { self.td_origin().from_cache().map_or(false, |v| v) }

}

