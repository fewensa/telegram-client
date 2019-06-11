
use rtdlib::types::*;
use crate::api::TDFB;


///  A HTTP transparent proxy server.  
#[derive(Debug, Clone)]
pub struct TGProxyTypeHttp {
  ///  Username for logging in; may be empty. 
  username: Option<String>,
  ///  Password for logging in; may be empty. 
  password: Option<String>,
  ///  Pass true, if the proxy supports only HTTP requests and doesn't support transparent TCP connections via HTTP CONNECT method. 
  http_only: Option<bool>,
  
}

impl TDFB for TGProxyTypeHttp {}

impl AsRef<TGProxyTypeHttp> for TGProxyTypeHttp {
  fn as_ref(&self) -> &TGProxyTypeHttp { self }
}

impl TGProxyTypeHttp {
  pub fn new() -> Self {
    Self {
      username: None,
      password: None,
      http_only: None,
      
    }
  }

  #[doc(hidden)]
  pub fn build(&self) -> ProxyTypeHttp {
    ProxyTypeHttp::builder()
      .username(self.username.clone())
      .password(self.password.clone())
      .http_only(self.http_only.clone())
      
      .build()
  }

  
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self {
    self.username = Some(username.as_ref().to_string());
    self
  }
  
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.password = Some(password.as_ref().to_string());
    self
  }
  
  pub fn http_only(&mut self, http_only: bool) -> &mut Self {
    self.http_only = Some(http_only);
    self
  }
  

}


///  An MTProto proxy server.  
#[derive(Debug, Clone)]
pub struct TGProxyTypeMtproto {
  ///  The proxy's secret in hexadecimal encoding. 
  secret: Option<String>,
  
}

impl TDFB for TGProxyTypeMtproto {}

impl AsRef<TGProxyTypeMtproto> for TGProxyTypeMtproto {
  fn as_ref(&self) -> &TGProxyTypeMtproto { self }
}

impl TGProxyTypeMtproto {
  pub fn new() -> Self {
    Self {
      secret: None,
      
    }
  }

  #[doc(hidden)]
  pub fn build(&self) -> ProxyTypeMtproto {
    ProxyTypeMtproto::builder()
      .secret(self.secret.clone())
      
      .build()
  }

  
  pub fn secret<S: AsRef<str>>(&mut self, secret: S) -> &mut Self {
    self.secret = Some(secret.as_ref().to_string());
    self
  }
  

}


///  A SOCKS5 proxy server.  
#[derive(Debug, Clone)]
pub struct TGProxyTypeSocks5 {
  ///  Username for logging in; may be empty. 
  username: Option<String>,
  ///  Password for logging in; may be empty. 
  password: Option<String>,
  
}

impl TDFB for TGProxyTypeSocks5 {}

impl AsRef<TGProxyTypeSocks5> for TGProxyTypeSocks5 {
  fn as_ref(&self) -> &TGProxyTypeSocks5 { self }
}

impl TGProxyTypeSocks5 {
  pub fn new() -> Self {
    Self {
      username: None,
      password: None,
      
    }
  }

  #[doc(hidden)]
  pub fn build(&self) -> ProxyTypeSocks5 {
    ProxyTypeSocks5::builder()
      .username(self.username.clone())
      .password(self.password.clone())
      
      .build()
  }

  
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self {
    self.username = Some(username.as_ref().to_string());
    self
  }
  
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.password = Some(password.as_ref().to_string());
    self
  }
  

}

