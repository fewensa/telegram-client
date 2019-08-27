use rtdlib::types::*;

use telegram_client::api::Api;

pub struct TProxy<'a> {
  api: &'a Api
}

impl<'a> TProxy<'a> {
  pub fn new(api: &'a Api) -> Self {
    Self { api }
  }

  pub fn add(&self) {
    let proxy = AddProxy::builder()
      .server("127.0.0.1")
      .port(1080)
      .enable(true)
      .type_(ProxyType::socks5(ProxyTypeSocks5::builder().build()))
      .build();
    self.api.send(proxy);
  }
}
