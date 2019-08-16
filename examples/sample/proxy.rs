use rtdlib::types::{AddProxy, ProxyTypeSocks5};

use telegram_client::api::Api;
use crate::exmlog;

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
      .type_(Box::new(ProxyTypeSocks5::builder().build()))
      .build();
    self.api.send(proxy);
  }
}
