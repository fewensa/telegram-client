use rtdlib::types as td_types;
use rtdlib::types::{
  RObject,
  NotificationGroupType
};

use crate::errors;
use crate::types::*;

impl TGScopeNotificationSettings {


  /// Time left before notifications will be unmuted, in seconds.
  pub fn mute_for(&self) -> i32 { self.td_origin().mute_for().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  /// The name of an audio file to be used for notification sounds; only applies to iOS applications.
  pub fn sound(&self) -> String { self.td_origin().sound().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  /// True, if message content should be displayed in notifications.
  pub fn show_preview(&self) -> bool { self.td_origin().show_preview().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  /// True, if notifications for incoming pinned messages will be created as for an ordinary unread message.
  pub fn disable_pinned_message_notifications(&self) -> bool { self.td_origin().disable_pinned_message_notifications().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  /// True, if notifications for messages with mentions will be created as for an ordinary unread message.
  pub fn disable_mention_notifications(&self) -> bool { self.td_origin().disable_mention_notifications().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }


}



impl TGUpdateScopeNotificationSettings {
  pub fn scope(&self) -> TGNotificationSettingsScope { TGNotificationSettingsScope::of(self.td_origin().scope().expect(&errors::data_fail_with_rtd(self.td_origin())[..])) }

  pub fn notification_settings(&self) -> TGScopeNotificationSettings {
    TGScopeNotificationSettings::from_json(self.td_origin().notification_settings().expect(&errors::data_fail_with_rtd(self.td_origin())[..]).to_json())
      .expect(&errors::data_fail_with_rtd(self.td_origin())[..])
  }
}


// todo unrelease
impl TGUpdateActiveNotifications {}

impl TGNotificationGroup {

  /// Unique persistent auto-incremented from 1 identifier of the notification group.
  pub fn id(&self) -> i32 { self.td_origin().id().map_or(1, |v| v) }

  pub fn type_(&self) -> TGNotificationGroupType { self.td_origin().type_().map(|v| TGNotificationGroupType::of(v)).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn chat_id(&self) -> i64 { self.td_origin().chat_id().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn total_count(&self) -> i32 { self.td_origin().total_count().map_or(0, |v| v) }

//  pub fn notifications(&self) -> Option<Vec<Notification>> { self.notifications.clone() }

}

#[derive(Debug, Clone)]
pub enum TGNotificationGroupType {
  Calls,
  Mentions,
  Messages,
  SecretChat,
}

impl TGNotificationGroupType {

  pub(crate) fn of(td: Box<td_types::NotificationGroupType>) -> Self {
    rtd_type_mapping!(
      NotificationGroupType,
      TGNotificationGroupType,
      RTDNotificationGroupTypeType,
      (NotificationGroupTypeCalls      , Calls      );
      (NotificationGroupTypeMentions   , Mentions   );
      (NotificationGroupTypeMessages   , Messages   );
      (NotificationGroupTypeSecretChat , SecretChat );
    )(td)
  }

  pub fn is_calls        (&self) -> bool { enum_is!(TGNotificationGroupType, Calls     )(self) }
  pub fn is_mentions     (&self) -> bool { enum_is!(TGNotificationGroupType, Mentions  )(self) }
  pub fn is_messages     (&self) -> bool { enum_is!(TGNotificationGroupType, Messages  )(self) }
  pub fn is_secret_chat  (&self) -> bool { enum_is!(TGNotificationGroupType, SecretChat)(self) }

  pub fn on_calls       <F: FnOnce()>(&self, fnc: F) -> &Self { enum_on!(TGNotificationGroupType, Calls     , || fnc())(self); self }
  pub fn on_mentions    <F: FnOnce()>(&self, fnc: F) -> &Self { enum_on!(TGNotificationGroupType, Mentions  , || fnc())(self); self }
  pub fn on_messages    <F: FnOnce()>(&self, fnc: F) -> &Self { enum_on!(TGNotificationGroupType, Messages  , || fnc())(self); self }
  pub fn on_secret_chat <F: FnOnce()>(&self, fnc: F) -> &Self { enum_on!(TGNotificationGroupType, SecretChat, || fnc())(self); self }
}


impl TGNotification {}

impl TGNotificationTypeNewCall {}

impl TGNotificationTypeNewMessage {}

impl TGNotificationTypeNewPushMessage {}

impl TGNotificationTypeNewSecretChat {}


