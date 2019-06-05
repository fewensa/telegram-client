use crate::types::t_update_user_status::TGUpdateUserStatus;
use rtdlib::types as td_types;
use crate::types::f_user::TGUserStatus;
use crate::errors;

impl TGUpdateUserStatus {

  pub fn user_id(&self) -> i32 { self.origin().user_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn status(&self) -> TGUserStatus {
    TGUserStatus::of(self.origin().status().expect(errors::TELEGRAM_DATA_FAIL))
  }

}
