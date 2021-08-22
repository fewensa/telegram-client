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
