use rtdlib::types as td_types;

use crate::errors;
use crate::types::{TGScopeNotificationSettings, TGUpdateScopeNotificationSettings, TGNotificationSettingsScope};
use rtdlib::types::RObject;

impl TGUpdateScopeNotificationSettings {
  pub fn scope(&self) -> TGNotificationSettingsScope { TGNotificationSettingsScope::of(self.td_origin().scope().expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn notification_settings(&self) -> TGScopeNotificationSettings {
    TGScopeNotificationSettings::from_json(self.td_origin().notification_settings().expect(errors::TELEGRAM_DATA_FAIL).to_json())
      .expect(errors::TELEGRAM_DATA_FAIL)
  }
}


