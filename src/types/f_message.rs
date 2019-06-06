use rtdlib::types as td_types;

use crate::errors;
use crate::types::t_message::TGMessage;

impl TGMessage {

  // todo

  pub fn id(&self) -> i64 { self.origin().id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn sender_user_id(&self) -> i32 { self.origin().sender_user_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn chat_id(&self) -> i64 { self.origin().chat_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn sending_state(&self) -> TGMessageSendingState { self.origin().sending_state().map(|v| TGMessageSendingState::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn is_outgoing(&self) -> bool { self.origin().is_outgoing().map_or(false, |v| v) }

  pub fn can_be_edited(&self) -> bool { self.origin().can_be_edited().map_or(false, |v| v) }

  pub fn can_be_forwarded(&self) -> bool { self.origin().can_be_forwarded().map_or(false, |v| v) }

  pub fn can_be_deleted_only_for_self(&self) -> bool { self.origin().can_be_deleted_only_for_self().map_or(false, |v| v) }

  pub fn can_be_deleted_for_all_users(&self) -> bool { self.origin().can_be_deleted_for_all_users().map_or(false, |v| v) }

  pub fn is_channel_post(&self) -> bool { self.origin().is_channel_post().map_or(false, |v| v) }

  pub fn contains_unread_mention(&self) -> bool { self.origin().contains_unread_mention().map_or(false, |v| v) }

  pub fn date(&self) -> i32 { self.origin().date().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn edit_date(&self) -> Option<i32> { self.origin().edit_date() }

  pub fn forward_info(&self) -> Option<td_types::MessageForwardInfo> { self.origin().forward_info() }

  pub fn reply_to_message_id(&self) -> Option<i64> { self.origin().reply_to_message_id() }

  pub fn ttl(&self) -> Option<i32> { self.origin().ttl() }

  pub fn ttl_expires_in(&self) -> Option<f64> { self.origin().ttl_expires_in() }

  pub fn via_bot_user_id(&self) -> Option<i32> { self.origin().via_bot_user_id() }

  pub fn author_signature(&self) -> Option<String> { self.origin().author_signature() }

  pub fn views(&self) -> Option<i32> { self.origin().views() }

  pub fn media_album_id(&self) -> Option<i64> {
    // https://core.telegram.org/tdlib/docs/classtd_1_1td__api_1_1message.html
    self.origin().media_album_id().map(|v| toolkit::number::as_i64(v).expect(errors::TELEGRAM_DATA_FAIL))
  }

  pub fn content(&self) -> Option<Box<td_types::MessageContent>> { self.origin().content() }

  pub fn reply_markup(&self) -> Option<Box<td_types::ReplyMarkup>> { self.origin().reply_markup() }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum TGMessageSendingState {
  Failed,
  Pending,
}

impl TGMessageSendingState {
  fn of(td: Box<td_types::MessageSendingState>) -> Self {
    match td_types::RTDMessageSendingStateType::of(td.to_json()) {
      Some(td_types::RTDMessageSendingStateType::MessageSendingStateFailed) => TGMessageSendingState::Failed,
      Some(td_types::RTDMessageSendingStateType::MessageSendingStatePending) => TGMessageSendingState::Pending,
      None => panic!(errors::TELEGRAM_DATA_FAIL)
    }
  }
}

