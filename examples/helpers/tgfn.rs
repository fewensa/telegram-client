use colored::Colorize;

use rtdlib::types::*;
use telegram_client::api::Api;

use crate::thelp;

pub fn type_phone_number(api: &Api) {
  let input = thelp::typed();
  api.event_api()
    .set_authentication_phone_number(
      SetAuthenticationPhoneNumber::builder()
        .phone_number(&input)
        .build()
    )
    .expect("failed to set authentication phone number");
  debug!("Set phone number [{}] {}", input.green(), "(If you copy log to anywhere, don't forget hide your phone number)".red());
}

pub fn type_authentication_code(api: &Api) {
  let code = thelp::typed();
  api.event_api()
    .check_authentication_code(CheckAuthenticationCode::builder().code(&code))
    .expect("failed to check authentication code");
  debug!("Set authentication code: {}", code);
}

#[allow(dead_code)]
pub fn type_and_register(api: &Api) {
  let first_name = thelp::typed_with_message("Please input first name:");
  let last_name = thelp::typed_with_message("Please input last name:");
  debug!("You name is {} {}", first_name, last_name);
  api.event_api()
    .register_user(RegisterUser::builder()
    .first_name(first_name)
    .last_name(last_name)
    .build())
    .expect("failed to register user");
}
