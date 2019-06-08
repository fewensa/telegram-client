use rtdlib::types as td_types;

use crate::errors;
use crate::types::TGChatNotificationSettings;

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
//      None => panic!(errors::TELEGRAM_DATA_FAIL)
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
}


impl TGChatNotificationSettings {
  pub fn use_default_mute_for(&self) -> bool { self.origin().use_default_mute_for().map_or(false, |v| v) }

  pub fn mute_for(&self) -> Option<i32> { self.origin().mute_for() }

  pub fn use_default_sound(&self) -> bool { self.origin().use_default_sound().map_or(false, |v| v) }

  pub fn sound(&self) -> Option<String> { self.origin().sound() }

  pub fn use_default_show_preview(&self) -> bool { self.origin().use_default_show_preview().map_or(false, |v| v) }

  pub fn show_preview(&self) -> bool { self.origin().show_preview().map_or(false, |v| v) }

  pub fn use_default_disable_pinned_message_notifications(&self) -> bool { self.origin().use_default_disable_pinned_message_notifications().map_or(false, |v| v) }

  pub fn disable_pinned_message_notifications(&self) -> bool { self.origin().disable_pinned_message_notifications().map_or(false, |v| v) }

  pub fn use_default_disable_mention_notifications(&self) -> bool { self.origin().use_default_disable_mention_notifications().map_or(false, |v| v) }

  pub fn disable_mention_notifications(&self) -> bool { self.origin().disable_mention_notifications().map_or(false, |v| v) }
}

