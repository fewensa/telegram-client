use rtdlib::types as td_types;

use telegram_client::api::Api;

use crate::{exmlog, thelp};

pub fn type_phone_number(api: &Api) {
  let input = thelp::typed();
  api.send(td_types::SetAuthenticationPhoneNumber::builder().phone_number(&input).build());
  debug!(exmlog::examples(), "Set phone number {}", input);
}

pub fn type_authentication_code(api: &Api) {
  let code = thelp::typed();
  api.send(td_types::CheckAuthenticationCode::builder().code(&code).build());
  debug!(exmlog::examples(), "Set authentication code: {}", code);
}

pub fn type_authentication_code_register(api: &Api) {
  let first_name = thelp::typed_with_message("Please input first name:");
  let last_name = thelp::typed_with_message("Please input last name:");
  let code = thelp::typed_with_message("Please type authentication code:");
  api.send(td_types::CheckAuthenticationCode::builder()
    .first_name(&first_name)
    .last_name(&last_name)
    .code(&code)
    .build());
  debug!(exmlog::examples(), "You name is {} {}, authentication: {}", first_name, last_name, code);
}

