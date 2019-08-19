use rtdlib::types as td_types;
use rtdlib::types::ChatType;
use rtdlib::types::RObject;
use rtdlib::types::NotificationSettingsScope;
use rtdlib::types::RTDChatActionType;
use rtdlib::types::ChatAction;

use crate::errors;
use crate::types::f_input_message::TGInputMessageContent;
use crate::types::t_chat::*;
use crate::types::t_message::TGMessage;


impl TGChat {
  pub fn id(&self) -> i64 { self.td_origin().id().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn type_(&self) -> TGChatType { self.td_origin().type_().map(|v| TGChatType::of(v)).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn title(&self) -> String { self.td_origin().title().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn photo(&self) -> Option<td_types::ChatPhoto> { self.td_origin().photo() }

  pub fn last_message(&self) -> Option<TGMessage> { self.td_origin().last_message().map(|v| TGMessage::from_json(v.to_json()).expect(&errors::data_fail_with_rtd(self.td_origin())[..])) }

  pub fn order(&self) -> i64 {
    // https://core.telegram.org/tdlib/docs/classtd_1_1td__api_1_1chat.html
    self.td_origin().order().map(|v| toolkit::number::as_i64(v).expect(&errors::data_fail_with_rtd(self.td_origin())[..])).expect(&errors::data_fail_with_rtd(self.td_origin())[..])
  }

  pub fn is_pinned(&self) -> bool { self.td_origin().is_pinned().map_or(false, |v| v) }

  pub fn is_marked_as_unread(&self) -> bool { self.td_origin().is_marked_as_unread().map_or(false, |v| v) }

  pub fn is_sponsored(&self) -> bool { self.td_origin().is_sponsored().map_or(false, |v| v) }

  pub fn can_be_deleted_only_for_self(&self) -> bool { self.td_origin().can_be_deleted_only_for_self().map_or(false, |v| v) }

  pub fn can_be_deleted_for_all_users(&self) -> bool { self.td_origin().can_be_deleted_for_all_users().map_or(false, |v| v) }

  pub fn can_be_reported(&self) -> bool { self.td_origin().can_be_reported().map_or(false, |v| v) }

  pub fn default_disable_notification(&self) -> bool { self.td_origin().default_disable_notification().map_or(false, |v| v) }

  pub fn unread_count(&self) -> i32 { self.td_origin().unread_count().map_or(0, |v| v) }

  pub fn last_read_inbox_message_id(&self) -> i64 { self.td_origin().last_read_inbox_message_id().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn last_read_outbox_message_id(&self) -> i64 { self.td_origin().last_read_outbox_message_id().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn unread_mention_count(&self) -> i32 { self.td_origin().unread_mention_count().map_or(0, |v| v) }

  pub fn notification_settings(&self) -> Option<TGChatNotificationSettings> {
    self.td_origin().notification_settings().map(|v| TGChatNotificationSettings::from_json(v.to_json()).expect(&errors::data_fail_with_rtd(self.td_origin())[..]))
  }

  pub fn pinned_message_id(&self) -> Option<i64> { self.td_origin().pinned_message_id() }

  pub fn reply_markup_message_id(&self) -> Option<i64> { self.td_origin().reply_markup_message_id() }

  pub fn draft_message(&self) -> Option<TGDraftMessage> { self.td_origin().draft_message().map(|v| TGDraftMessage::from_json(v.to_json()).expect(&errors::data_fail_with_rtd(self.td_origin())[..])) }

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


impl TGChatTypeBasicGroup {
  pub fn basic_group_id(&self) -> i32 { self.td_origin().basic_group_id().map(|v| v).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }
}

impl TGChatTypePrivate {
  pub fn user_id(&self) -> i32 { self.td_origin().user_id().map(|v| v).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }
}

impl TGChatTypeSecret {
  pub fn secret_chat_id(&self) -> i32 { self.td_origin().secret_chat_id().map(|v| v).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn user_id(&self) -> i32 { self.td_origin().user_id().map(|v| v).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }
}

impl TGChatTypeSupergroup {
  pub fn supergroup_id(&self) -> i32 { self.td_origin().supergroup_id().map(|v| v).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn is_channel(&self) -> bool { self.td_origin().is_channel().map_or(false, |v| v) }
}


/// This class is an abstract base class. Describes the types of chats to which notification settings are applied.
pub enum TGNotificationSettingsScope {
  ChannelChats,
  GroupChats,
  PrivateChats,
}

impl TGNotificationSettingsScope {
  pub(crate) fn of(td: Box<td_types::NotificationSettingsScope>) -> Self {
//    match td_types::RTDNotificationSettingsScopeType::of(td.td_name()) {
//      Some(td_types::RTDNotificationSettingsScopeType::NotificationSettingsScopeChannelChats) => TGNotificationSettingsScope::ChannelChats,
//      Some(td_types::RTDNotificationSettingsScopeType::NotificationSettingsScopeGroupChats) => TGNotificationSettingsScope::GroupChats,
//      Some(td_types::RTDNotificationSettingsScopeType::NotificationSettingsScopePrivateChats) => TGNotificationSettingsScope::PrivateChats,
//      None => panic!(&errors::data_fail_with_rtd(self.td_origin())[..])
//    }
    rtd_type_mapping!(
      NotificationSettingsScope,
      TGNotificationSettingsScope,
      RTDNotificationSettingsScopeType,
      (NotificationSettingsScopeChannelChats,  ChannelChats  );
      (NotificationSettingsScopeGroupChats  ,  GroupChats    );
      (NotificationSettingsScopePrivateChats,  PrivateChats  );
    )(td)
  }

//  channel_chats
//  group_chats
//  private_chats

  pub fn is_channel_chats(&self) -> bool { enum_is!(TGNotificationSettingsScope, ChannelChats )(self) }
  pub fn is_group_chats(&self) -> bool { enum_is!(TGNotificationSettingsScope, GroupChats   )(self) }
  pub fn is_private_chats(&self) -> bool { enum_is!(TGNotificationSettingsScope, PrivateChats )(self) }

  pub fn on_channel_chats <F: FnOnce()>(&self, fnc: F) -> &Self { enum_on!(TGNotificationSettingsScope,  ChannelChats , || fnc()  )(self); self }
  pub fn on_group_chats   <F: FnOnce()>(&self, fnc: F) -> &Self { enum_on!(TGNotificationSettingsScope,  GroupChats   , || fnc()  )(self); self }
  pub fn on_private_chats <F: FnOnce()>(&self, fnc: F) -> &Self { enum_on!(TGNotificationSettingsScope,  PrivateChats , || fnc()  )(self); self }

}


impl TGChatNotificationSettings {
  pub fn use_default_mute_for(&self) -> bool { self.td_origin().use_default_mute_for().map_or(false, |v| v) }

  pub fn mute_for(&self) -> Option<i32> { self.td_origin().mute_for() }

  pub fn use_default_sound(&self) -> bool { self.td_origin().use_default_sound().map_or(false, |v| v) }

  pub fn sound(&self) -> Option<String> { self.td_origin().sound() }

  pub fn use_default_show_preview(&self) -> bool { self.td_origin().use_default_show_preview().map_or(false, |v| v) }

  pub fn show_preview(&self) -> bool { self.td_origin().show_preview().map_or(false, |v| v) }

  pub fn use_default_disable_pinned_message_notifications(&self) -> bool { self.td_origin().use_default_disable_pinned_message_notifications().map_or(false, |v| v) }

  pub fn disable_pinned_message_notifications(&self) -> bool { self.td_origin().disable_pinned_message_notifications().map_or(false, |v| v) }

  pub fn use_default_disable_mention_notifications(&self) -> bool { self.td_origin().use_default_disable_mention_notifications().map_or(false, |v| v) }

  pub fn disable_mention_notifications(&self) -> bool { self.td_origin().disable_mention_notifications().map_or(false, |v| v) }
}


impl TGUpdateChatLastMessage {
  pub fn chat_id(&self) -> i64 { self.td_origin().chat_id().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn last_message(&self) -> Option<TGMessage> { self.td_origin().last_message().map(|v| TGMessage::from_json(v.to_json()).expect(&errors::data_fail_with_rtd(self.td_origin())[..])) }

  pub fn order(&self) -> i64 { self.td_origin().order().map(|v| toolkit::number::as_i64(v).expect(&errors::data_fail_with_rtd(self.td_origin())[..])).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }
}


impl TGUpdateNewChat {
  pub fn chat(&self) -> TGChat { self.td_origin().chat().map(|v| TGChat::from_json(v.to_json()).expect(&errors::data_fail_with_rtd(self.td_origin())[..])).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }
}


impl TGUpdateUserChatAction {

  pub fn chat_id(&self) -> i64 { self.td_origin().chat_id().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn user_id(&self) -> i32 { self.td_origin().user_id().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn action(&self) -> TGChatAction { self.td_origin().action().map(|v| TGChatAction::of(v)).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

}


/// This class is an abstract base class. Describes the type of a chat.
#[derive(Debug, Clone)]
pub enum TGChatAction {
  Cancel                (TGChatActionCancel                ),
  ChoosingContact       (TGChatActionChoosingContact       ),
  ChoosingLocation      (TGChatActionChoosingLocation      ),
  RecordingVideo        (TGChatActionRecordingVideo        ),
  RecordingVideoNote    (TGChatActionRecordingVideoNote    ),
  RecordingVoiceNote    (TGChatActionRecordingVoiceNote    ),
  StartPlayingGame      (TGChatActionStartPlayingGame      ),
  Typing                (TGChatActionTyping                ),
  UploadingDocument     (TGChatActionUploadingDocument     ),
  UploadingPhoto        (TGChatActionUploadingPhoto        ),
  UploadingVideo        (TGChatActionUploadingVideo        ),
  UploadingVideoNote    (TGChatActionUploadingVideoNote    ),
  UploadingVoiceNote    (TGChatActionUploadingVoiceNote    ),
}

impl TGChatAction {
  pub(crate) fn of(td: Box<td_types::ChatAction>) -> Self {
    tuple_rtd_type_mapping!(
      ChatAction,
      TGChatAction,
      RTDChatActionType,
      (ChatActionCancel             ,  Cancel             , TGChatActionCancel              );
      (ChatActionChoosingContact    ,  ChoosingContact    , TGChatActionChoosingContact     );
      (ChatActionChoosingLocation   ,  ChoosingLocation   , TGChatActionChoosingLocation    );
      (ChatActionRecordingVideo     ,  RecordingVideo     , TGChatActionRecordingVideo      );
      (ChatActionRecordingVideoNote ,  RecordingVideoNote , TGChatActionRecordingVideoNote  );
      (ChatActionRecordingVoiceNote ,  RecordingVoiceNote , TGChatActionRecordingVoiceNote  );
      (ChatActionStartPlayingGame   ,  StartPlayingGame   , TGChatActionStartPlayingGame    );
      (ChatActionTyping             ,  Typing             , TGChatActionTyping              );
      (ChatActionUploadingDocument  ,  UploadingDocument  , TGChatActionUploadingDocument   );
      (ChatActionUploadingPhoto     ,  UploadingPhoto     , TGChatActionUploadingPhoto      );
      (ChatActionUploadingVideo     ,  UploadingVideo     , TGChatActionUploadingVideo      );
      (ChatActionUploadingVideoNote ,  UploadingVideoNote , TGChatActionUploadingVideoNote  );
      (ChatActionUploadingVoiceNote ,  UploadingVoiceNote , TGChatActionUploadingVoiceNote  );
    )(td)
  }

  pub fn is_cancel                 (&self) -> bool { tuple_enum_is!(TGChatAction, Cancel            )(self) }
  pub fn is_choosing_contact       (&self) -> bool { tuple_enum_is!(TGChatAction, ChoosingContact   )(self) }
  pub fn is_choosing_location      (&self) -> bool { tuple_enum_is!(TGChatAction, ChoosingLocation  )(self) }
  pub fn is_recording_video        (&self) -> bool { tuple_enum_is!(TGChatAction, RecordingVideo    )(self) }
  pub fn is_recording_video_note   (&self) -> bool { tuple_enum_is!(TGChatAction, RecordingVideoNote)(self) }
  pub fn is_recording_voice_note   (&self) -> bool { tuple_enum_is!(TGChatAction, RecordingVoiceNote)(self) }
  pub fn is_start_playing_game     (&self) -> bool { tuple_enum_is!(TGChatAction, StartPlayingGame  )(self) }
  pub fn is_typing                 (&self) -> bool { tuple_enum_is!(TGChatAction, Typing            )(self) }
  pub fn is_uploading_document     (&self) -> bool { tuple_enum_is!(TGChatAction, UploadingDocument )(self) }
  pub fn is_uploading_photo        (&self) -> bool { tuple_enum_is!(TGChatAction, UploadingPhoto    )(self) }
  pub fn is_uploading_video        (&self) -> bool { tuple_enum_is!(TGChatAction, UploadingVideo    )(self) }
  pub fn is_uploading_video_note   (&self) -> bool { tuple_enum_is!(TGChatAction, UploadingVideoNote)(self) }
  pub fn is_uploading_voice_note   (&self) -> bool { tuple_enum_is!(TGChatAction, UploadingVoiceNote)(self) }

  pub fn on_cancel                <F: FnOnce(&TGChatActionCancel             )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, Cancel            , |t| fnc(t))(self); self }
  pub fn on_choosing_contact      <F: FnOnce(&TGChatActionChoosingContact    )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, ChoosingContact   , |t| fnc(t))(self); self }
  pub fn on_choosing_location     <F: FnOnce(&TGChatActionChoosingLocation   )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, ChoosingLocation  , |t| fnc(t))(self); self }
  pub fn on_recording_video       <F: FnOnce(&TGChatActionRecordingVideo     )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, RecordingVideo    , |t| fnc(t))(self); self }
  pub fn on_recording_video_note  <F: FnOnce(&TGChatActionRecordingVideoNote )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, RecordingVideoNote, |t| fnc(t))(self); self }
  pub fn on_recording_voice_note  <F: FnOnce(&TGChatActionRecordingVoiceNote )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, RecordingVoiceNote, |t| fnc(t))(self); self }
  pub fn on_start_playing_game    <F: FnOnce(&TGChatActionStartPlayingGame   )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, StartPlayingGame  , |t| fnc(t))(self); self }
  pub fn on_typing                <F: FnOnce(&TGChatActionTyping             )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, Typing            , |t| fnc(t))(self); self }
  pub fn on_uploading_document    <F: FnOnce(&TGChatActionUploadingDocument  )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, UploadingDocument , |t| fnc(t))(self); self }
  pub fn on_uploading_photo       <F: FnOnce(&TGChatActionUploadingPhoto     )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, UploadingPhoto    , |t| fnc(t))(self); self }
  pub fn on_uploading_video       <F: FnOnce(&TGChatActionUploadingVideo     )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, UploadingVideo    , |t| fnc(t))(self); self }
  pub fn on_uploading_video_note  <F: FnOnce(&TGChatActionUploadingVideoNote )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, UploadingVideoNote, |t| fnc(t))(self); self }
  pub fn on_uploading_voice_note  <F: FnOnce(&TGChatActionUploadingVoiceNote )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGChatAction, UploadingVoiceNote, |t| fnc(t))(self); self }


}

impl TGChatActionCancel {}

impl TGChatActionChoosingContact {}

impl TGChatActionChoosingLocation {}

impl TGChatActionRecordingVideo {}

impl TGChatActionRecordingVideoNote {}

impl TGChatActionRecordingVoiceNote {}

impl TGChatActionStartPlayingGame {}

impl TGChatActionTyping {}

impl TGChatActionUploadingDocument {
  pub fn progress(&self) -> i32 { self.td_origin().progress().map_or(0, |v| v) }
}

impl TGChatActionUploadingPhoto {
  pub fn progress(&self) -> i32 { self.td_origin().progress().map_or(0, |v| v) }
}

impl TGChatActionUploadingVideo {
  pub fn progress(&self) -> i32 { self.td_origin().progress().map_or(0, |v| v) }
}

impl TGChatActionUploadingVideoNote {
  pub fn progress(&self) -> i32 { self.td_origin().progress().map_or(0, |v| v) }
}

impl TGChatActionUploadingVoiceNote {
  pub fn progress(&self) -> i32 { self.td_origin().progress().map_or(0, |v| v) }
}




