use rtdlib::types::ProxyType;

use crate::api::*;

impl _TGAddProxyBuilder {
  pub fn http<T: AsRef<TGProxyTypeHttp>>(&mut self, proxy: T) -> &mut Self {
    self._type_(Box::new(proxy.as_ref().td_origin().clone()))
  }

  pub fn mtproto<T: AsRef<TGProxyTypeMtproto>>(&mut self, proxy: T) -> &mut Self {
    self._type_(Box::new(proxy.as_ref().td_origin().clone()))
  }

  pub fn socks5<T: AsRef<TGProxyTypeSocks5>>(&mut self, proxy: T) -> &mut Self {
    self._type_(Box::new(proxy.as_ref().td_origin().clone()))
  }
}



