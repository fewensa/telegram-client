use crate::types::t_update_user::TGUpdateUser;
use crate::types::TGUser;
use crate::errors;
use rtdlib::types::RObject;

impl TGUpdateUser {

  pub fn user(&self) -> TGUser { TGUser::from_json(self.td_origin().user().expect(errors::TELEGRAM_DATA_FAIL).to_json()).expect(errors::TELEGRAM_DATA_FAIL) }

}
