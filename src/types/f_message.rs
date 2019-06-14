use rtdlib::types as td_types;
use rtdlib::types::MessageSendingState;
use rtdlib::types::RObject;

use crate::errors;
use crate::types::t_message::TGMessage;
use crate::types::TGMessageContent;
use crate::types::TGUpdateMessageEdited;
use crate::types::TGMessageForwardInfo;
use crate::types::TGReplyMarkup;

impl TGMessage {

  pub fn id(&self) -> i64 { self.td_origin().id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn sender_user_id(&self) -> i32 { self.td_origin().sender_user_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn chat_id(&self) -> i64 { self.td_origin().chat_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn sending_state(&self) -> Option<TGMessageSendingState> { self.td_origin().sending_state().map(|v| TGMessageSendingState::of(v)) }

  pub fn is_outgoing(&self) -> bool { self.td_origin().is_outgoing().map_or(false, |v| v) }

  pub fn can_be_edited(&self) -> bool { self.td_origin().can_be_edited().map_or(false, |v| v) }

  pub fn can_be_forwarded(&self) -> bool { self.td_origin().can_be_forwarded().map_or(false, |v| v) }

  pub fn can_be_deleted_only_for_self(&self) -> bool { self.td_origin().can_be_deleted_only_for_self().map_or(false, |v| v) }

  pub fn can_be_deleted_for_all_users(&self) -> bool { self.td_origin().can_be_deleted_for_all_users().map_or(false, |v| v) }

  pub fn is_channel_post(&self) -> bool { self.td_origin().is_channel_post().map_or(false, |v| v) }

  pub fn contains_unread_mention(&self) -> bool { self.td_origin().contains_unread_mention().map_or(false, |v| v) }

  pub fn date(&self) -> i32 { self.td_origin().date().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn edit_date(&self) -> Option<i32> { self.td_origin().edit_date() }

  pub fn forward_info(&self) -> Option<TGMessageForwardInfo> { self.td_origin().forward_info().map(|v| TGMessageForwardInfo::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn reply_to_message_id(&self) -> Option<i64> { self.td_origin().reply_to_message_id() }

  pub fn ttl(&self) -> Option<i32> { self.td_origin().ttl() }

  pub fn ttl_expires_in(&self) -> Option<f64> { self.td_origin().ttl_expires_in() }

  pub fn via_bot_user_id(&self) -> Option<i32> { self.td_origin().via_bot_user_id() }

  pub fn author_signature(&self) -> Option<String> { self.td_origin().author_signature() }

  pub fn views(&self) -> Option<i32> { self.td_origin().views() }

  pub fn media_album_id(&self) -> Option<i64> {
    // https://core.telegram.org/tdlib/docs/classtd_1_1td__api_1_1message.html
    self.td_origin().media_album_id().map(|v| toolkit::number::as_i64(v).expect(errors::TELEGRAM_DATA_FAIL))
  }

  pub fn content(&self) -> TGMessageContent { self.td_origin().content().map(|v| TGMessageContent::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn reply_markup(&self) -> Option<TGReplyMarkup> { self.td_origin().reply_markup().map(|v| TGReplyMarkup::of(v)) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum TGMessageSendingState {
  Failed,
  Pending,
}

impl TGMessageSendingState {
  fn of(td: Box<td_types::MessageSendingState>) -> Self {
    rtd_type_mapping!(
      MessageSendingState,
      TGMessageSendingState,
      RTDMessageSendingStateType,
      (MessageSendingStateFailed   , Failed );
      (MessageSendingStatePending  , Pending);
    )(td)
  }

  pub fn is_failed(&self) -> bool {
    enum_is!(TGMessageSendingState, Failed)(self)
  }

  pub fn is_pending(&self) -> bool {
    enum_is!(TGMessageSendingState, Pending)(self)
  }
}


impl TGUpdateMessageEdited {

  pub fn chat_id(&self) -> i64 { self.td_origin().chat_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn message_id(&self) -> i64 { self.td_origin().message_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn edit_date(&self) -> i32 { self.td_origin().edit_date().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn reply_markup(&self) -> Option<TGReplyMarkup> { self.td_origin().reply_markup().map(|v| TGReplyMarkup::of(v)) }

}
