use rtdlib::types as td_types;

use crate::errors;
use crate::types::{TGScopeNotificationSettings, TGUpdateScopeNotificationSettings};
use rtdlib::types::RObject;

impl TGUpdateScopeNotificationSettings {
  pub fn scope(&self) -> TGNotificationSettingsScope { TGNotificationSettingsScope::of(self.origin().scope().expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn notification_settings(&self) -> TGScopeNotificationSettings {
    TGScopeNotificationSettings::from_json(self.origin().notification_settings().expect(errors::TELEGRAM_DATA_FAIL).to_json())
      .expect(errors::TELEGRAM_DATA_FAIL)
  }
}

/// This class is an abstract base class. Describes the types of chats to which notification settings are applied.
pub enum TGNotificationSettingsScope {
  ChannelChats,
  GroupChats,
  PrivateChats,
}

impl TGNotificationSettingsScope {
  fn of(td: Box<td_types::NotificationSettingsScope>) -> Self {
    match td_types::RTDNotificationSettingsScopeType::of(td.td_name()) {
      Some(td_types::RTDNotificationSettingsScopeType::NotificationSettingsScopeChannelChats) => TGNotificationSettingsScope::ChannelChats,
      Some(td_types::RTDNotificationSettingsScopeType::NotificationSettingsScopeGroupChats) => TGNotificationSettingsScope::GroupChats,
      Some(td_types::RTDNotificationSettingsScopeType::NotificationSettingsScopePrivateChats) => TGNotificationSettingsScope::PrivateChats,
      None => panic!(errors::TELEGRAM_DATA_FAIL)
    }
  }
}



