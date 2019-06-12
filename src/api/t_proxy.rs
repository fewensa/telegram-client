
use rtdlib::types::*;
use crate::types::*;
use crate::api::TDFB;


#[doc(hidden)] pub struct _TGProxyTypeHttpBuilder { inner: TGProxyTypeHttp }

impl _TGProxyTypeHttpBuilder {

  pub fn build(&self) -> TGProxyTypeHttp { self.inner.clone() }

  ///  Username for logging in; may be empty. 
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self {
    self.inner.td_origin_mut()._set_username(username.as_ref().to_string());
    self
  }
  ///  Password for logging in; may be empty. 
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_password(password.as_ref().to_string());
    self
  }
  ///  Pass true, if the proxy supports only HTTP requests and doesn't support transparent TCP connections via HTTP CONNECT method. 
  pub fn http_only(&mut self, http_only: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_http_only(http_only);
    self
  }
  

  
}

///  A HTTP transparent proxy server.  
#[derive(Debug, Clone)]
pub struct TGProxyTypeHttp {
  inner: ProxyTypeHttp
}

impl TDFB for TGProxyTypeHttp {}

impl AsRef<TGProxyTypeHttp> for TGProxyTypeHttp {
  fn as_ref(&self) -> &TGProxyTypeHttp { self }
}

impl AsRef<TGProxyTypeHttp> for _TGProxyTypeHttpBuilder {
  fn as_ref(&self) -> &TGProxyTypeHttp { &self.inner }
}

impl TGProxyTypeHttp {
  pub fn builder() -> _TGProxyTypeHttpBuilder {
    _TGProxyTypeHttpBuilder { inner: Self::new(ProxyTypeHttp::_new()) }
  }

  pub fn new(inner: ProxyTypeHttp) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ProxyTypeHttp { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ProxyTypeHttp { &mut self.inner }
}


#[doc(hidden)] pub struct _TGProxyTypeMtprotoBuilder { inner: TGProxyTypeMtproto }

impl _TGProxyTypeMtprotoBuilder {

  pub fn build(&self) -> TGProxyTypeMtproto { self.inner.clone() }

  ///  The proxy's secret in hexadecimal encoding. 
  pub fn secret<S: AsRef<str>>(&mut self, secret: S) -> &mut Self {
    self.inner.td_origin_mut()._set_secret(secret.as_ref().to_string());
    self
  }
  

  
}

///  An MTProto proxy server.  
#[derive(Debug, Clone)]
pub struct TGProxyTypeMtproto {
  inner: ProxyTypeMtproto
}

impl TDFB for TGProxyTypeMtproto {}

impl AsRef<TGProxyTypeMtproto> for TGProxyTypeMtproto {
  fn as_ref(&self) -> &TGProxyTypeMtproto { self }
}

impl AsRef<TGProxyTypeMtproto> for _TGProxyTypeMtprotoBuilder {
  fn as_ref(&self) -> &TGProxyTypeMtproto { &self.inner }
}

impl TGProxyTypeMtproto {
  pub fn builder() -> _TGProxyTypeMtprotoBuilder {
    _TGProxyTypeMtprotoBuilder { inner: Self::new(ProxyTypeMtproto::_new()) }
  }

  pub fn new(inner: ProxyTypeMtproto) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ProxyTypeMtproto { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ProxyTypeMtproto { &mut self.inner }
}


#[doc(hidden)] pub struct _TGProxyTypeSocks5Builder { inner: TGProxyTypeSocks5 }

impl _TGProxyTypeSocks5Builder {

  pub fn build(&self) -> TGProxyTypeSocks5 { self.inner.clone() }

  ///  Username for logging in; may be empty. 
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self {
    self.inner.td_origin_mut()._set_username(username.as_ref().to_string());
    self
  }
  ///  Password for logging in; may be empty. 
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_password(password.as_ref().to_string());
    self
  }
  

  
}

///  A SOCKS5 proxy server.  
#[derive(Debug, Clone)]
pub struct TGProxyTypeSocks5 {
  inner: ProxyTypeSocks5
}

impl TDFB for TGProxyTypeSocks5 {}

impl AsRef<TGProxyTypeSocks5> for TGProxyTypeSocks5 {
  fn as_ref(&self) -> &TGProxyTypeSocks5 { self }
}

impl AsRef<TGProxyTypeSocks5> for _TGProxyTypeSocks5Builder {
  fn as_ref(&self) -> &TGProxyTypeSocks5 { &self.inner }
}

impl TGProxyTypeSocks5 {
  pub fn builder() -> _TGProxyTypeSocks5Builder {
    _TGProxyTypeSocks5Builder { inner: Self::new(ProxyTypeSocks5::_new()) }
  }

  pub fn new(inner: ProxyTypeSocks5) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ProxyTypeSocks5 { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ProxyTypeSocks5 { &mut self.inner }
}

