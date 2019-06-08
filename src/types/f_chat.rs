use rtdlib::types as td_types;
use rtdlib::types::ChatType;
use rtdlib::types::RObject;

use crate::errors;
use crate::types::f_input_message::TGInputMessageContent;
use crate::types::t_chat::*;
use crate::types::t_chat_type::*;
use crate::types::t_message::TGMessage;
use crate::types::TGChatNotificationSettings;

impl TGChat {
  pub fn id(&self) -> i64 { self.td_origin().id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn type_(&self) -> TGChatType { self.td_origin().type_().map(|v| TGChatType::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn title(&self) -> String { self.td_origin().title().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn photo(&self) -> Option<td_types::ChatPhoto> { self.td_origin().photo() }

  pub fn last_message(&self) -> Option<TGMessage> { self.td_origin().last_message().map(|v| TGMessage::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn order(&self) -> i64 {
    // https://core.telegram.org/tdlib/docs/classtd_1_1td__api_1_1chat.html
    self.td_origin().order().map(|v| toolkit::number::as_i64(v).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL)
  }

  pub fn is_pinned(&self) -> bool { self.td_origin().is_pinned().map_or(false, |v| v) }

  pub fn is_marked_as_unread(&self) -> bool { self.td_origin().is_marked_as_unread().map_or(false, |v| v) }

  pub fn is_sponsored(&self) -> bool { self.td_origin().is_sponsored().map_or(false, |v| v) }

  pub fn can_be_deleted_only_for_self(&self) -> bool { self.td_origin().can_be_deleted_only_for_self().map_or(false, |v| v) }

  pub fn can_be_deleted_for_all_users(&self) -> bool { self.td_origin().can_be_deleted_for_all_users().map_or(false, |v| v) }

  pub fn can_be_reported(&self) -> bool { self.td_origin().can_be_reported().map_or(false, |v| v) }

  pub fn default_disable_notification(&self) -> bool { self.td_origin().default_disable_notification().map_or(false, |v| v) }

  pub fn unread_count(&self) -> i32 { self.td_origin().unread_count().map_or(0, |v| v) }

  pub fn last_read_inbox_message_id(&self) -> i64 { self.td_origin().last_read_inbox_message_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn last_read_outbox_message_id(&self) -> i64 { self.td_origin().last_read_outbox_message_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn unread_mention_count(&self) -> i32 { self.td_origin().unread_mention_count().map_or(0, |v| v) }

  pub fn notification_settings(&self) -> Option<TGChatNotificationSettings> {
    self.td_origin().notification_settings().map(|v| TGChatNotificationSettings::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
  }

  pub fn pinned_message_id(&self) -> Option<i64> { self.td_origin().pinned_message_id() }

  pub fn reply_markup_message_id(&self) -> Option<i64> { self.td_origin().reply_markup_message_id() }

  pub fn draft_message(&self) -> Option<TGDraftMessage> { self.td_origin().draft_message().map(|v| TGDraftMessage::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn client_data(&self) -> Option<String> { self.td_origin().client_data() }

  pub fn is_basic_group(&self) -> bool {
    self.type_().is_basic_group()
  }

  pub fn is_private(&self) -> bool {
    self.type_().is_private()
  }

  pub fn is_secret(&self) -> bool {
    self.type_().is_secret()
  }

  pub fn is_supergroup(&self) -> bool {
    self.type_().is_supergroup()
  }
}


/// This class is an abstract base class. Describes the type of a chat.
#[derive(Debug, Clone)]
pub enum TGChatType {
  BasicGroup(TGChatTypeBasicGroup),
  Private(TGChatTypePrivate),
  Secret(TGChatTypeSecret),
  Supergroup(TGChatTypeSupergroup),
}

impl TGChatType {
  fn of(td: Box<td_types::ChatType>) -> Self {
    tuple_rtd_type_mapping!(
      ChatType,
      TGChatType,
      RTDChatTypeType,
      (ChatTypeBasicGroup,  BasicGroup,  TGChatTypeBasicGroup);
      (ChatTypePrivate   ,  Private   ,  TGChatTypePrivate   );
      (ChatTypeSecret    ,  Secret    ,  TGChatTypeSecret    );
      (ChatTypeSupergroup,  Supergroup,  TGChatTypeSupergroup);
    )(td)
  }

//  basic_group
//  private
//  secret
//  supergroup


  pub fn is_basic_group(&self) -> bool { tuple_enum_is!(TGChatType, BasicGroup)(self) }
  pub fn is_private(&self) -> bool { tuple_enum_is!(TGChatType, Private   )(self) }
  pub fn is_secret(&self) -> bool { tuple_enum_is!(TGChatType, Secret    )(self) }
  pub fn is_supergroup(&self) -> bool { tuple_enum_is!(TGChatType, Supergroup)(self) }


  pub fn on_basic_group<F: FnOnce(&TGChatTypeBasicGroup)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGChatType, BasicGroup, |t| fnc(t))(self);
    self
  }
  pub fn on_private<F: FnOnce(&TGChatTypePrivate)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGChatType, Private   , |t| fnc(t))(self);
    self
  }
  pub fn on_secret<F: FnOnce(&TGChatTypeSecret)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGChatType, Secret    , |t| fnc(t))(self);
    self
  }
  pub fn on_supergroup<F: FnOnce(&TGChatTypeSupergroup)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGChatType, Supergroup, |t| fnc(t))(self);
    self
  }
}


impl TGDraftMessage {
  pub fn reply_to_message_id(&self) -> Option<i64> { self.td_origin().reply_to_message_id().filter(|v| v != &0) }

  pub fn input_message_text(&self) -> Option<TGInputMessageContent> { self.td_origin().input_message_text().map(|v| TGInputMessageContent::of(v)) }
}




