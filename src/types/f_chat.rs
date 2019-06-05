use crate::types::t_chat::TGChat;
use crate::errors;
use rtdlib::types as td_types;
use crate::types::t_message::TGMessage;

impl TGChat {

  // todo
  pub fn id(&self) -> i64 { self.origin().id().expect(errors::TELEGRAM_DATA_FAIL) }

//  pub fn type_(&self) -> Option<Box<td_types::ChatType>> { self.type_.clone() }
//
//  pub fn title(&self) -> Option<String> { self.title.clone() }
//
//  pub fn photo(&self) -> Option<td_types::ChatPhoto> { self.photo.clone() }
//
//  pub fn last_message(&self) -> TGMessage { self.last_message.clone() }
//
//  pub fn order(&self) -> Option<String> { self.order.clone() }
//
//  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
//
//  pub fn is_marked_as_unread(&self) -> Option<bool> { self.is_marked_as_unread.clone() }
//
//  pub fn is_sponsored(&self) -> Option<bool> { self.is_sponsored.clone() }
//
//  pub fn can_be_deleted_only_for_self(&self) -> Option<bool> { self.can_be_deleted_only_for_self.clone() }
//
//  pub fn can_be_deleted_for_all_users(&self) -> Option<bool> { self.can_be_deleted_for_all_users.clone() }
//
//  pub fn can_be_reported(&self) -> Option<bool> { self.can_be_reported.clone() }
//
//  pub fn default_disable_notification(&self) -> Option<bool> { self.default_disable_notification.clone() }
//
//  pub fn unread_count(&self) -> Option<i32> { self.unread_count.clone() }
//
//  pub fn last_read_inbox_message_id(&self) -> Option<i64> { self.last_read_inbox_message_id.clone() }
//
//  pub fn last_read_outbox_message_id(&self) -> Option<i64> { self.last_read_outbox_message_id.clone() }
//
//  pub fn unread_mention_count(&self) -> Option<i32> { self.unread_mention_count.clone() }
//
//  pub fn notification_settings(&self) -> Option<td_types::ChatNotificationSettings> { self.notification_settings.clone() }
//
//  pub fn pinned_message_id(&self) -> Option<i64> { self.pinned_message_id.clone() }
//
//  pub fn reply_markup_message_id(&self) -> Option<i64> { self.reply_markup_message_id.clone() }
//
//  pub fn draft_message(&self) -> Option<td_types::DraftMessage> { self.draft_message.clone() }
//
//  pub fn client_data(&self) -> Option<String> { self.client_data.clone() }

}

/// This class is an abstract base class. Describes the type of a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum TGChatType {
  BasicGroup,
  Private,
  Secret,
  Supergroup
}



