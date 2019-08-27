use telegram_client::api::Api;
use rtdlib::types::*;

use crate::{thelp};

pub fn type_phone_number(api: &Api) {
  let input = thelp::typed();
  api.set_authentication_phone_number(SetAuthenticationPhoneNumber::builder().phone_number(&input).build());
  debug!("Set phone number {}", input);
}

pub fn type_authentication_code(api: &Api) {
  let code = thelp::typed();
  api.check_authentication_code(CheckAuthenticationCode::builder().code(&code));
  debug!("Set authentication code: {}", code);
}

pub fn type_authentication_code_register(api: &Api) {
  let first_name = thelp::typed_with_message("Please input first name:");
  let last_name = thelp::typed_with_message("Please input last name:");
  let code = thelp::typed_with_message("Please type authentication code:");
  api.check_authentication_code(CheckAuthenticationCode::builder()
    .first_name(&first_name)
    .last_name(&last_name)
    .code(&code)
    .build());
  debug!("You name is {} {}, authentication: {}", first_name, last_name, code);
}

