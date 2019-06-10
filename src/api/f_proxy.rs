use rtdlib::types::ProxyType;

use crate::api::*;

impl TGAddProxy {
  pub fn http<T: AsRef<TGProxyTypeHttp>>(&mut self, proxy: T) -> &mut Self {
    self._type_(Box::new(proxy.as_ref().build()))
  }

  pub fn mtproto<T: AsRef<TGProxyTypeMtproto>>(&mut self, proxy: T) -> &mut Self {
    self._type_(Box::new(proxy.as_ref().build()))
  }

  pub fn socks5<T: AsRef<TGProxyTypeSocks5>>(&mut self, proxy: T) -> &mut Self {
    self._type_(Box::new(proxy.as_ref().build()))
  }
}



