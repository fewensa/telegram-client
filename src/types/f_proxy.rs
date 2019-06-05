use rtdlib::types as td_types;

use crate::types::t_proxy::TGProxy;

impl TGProxy {

  pub fn id(&self) -> Option<i32> { self.origin().id() }

  pub fn server(&self) -> Option<String> { self.origin().server() }

  pub fn port(&self) -> Option<i32> { self.origin().port() }

  pub fn last_used_date(&self) -> Option<i32> { self.origin().last_used_date() }

  pub fn is_enabled(&self) -> Option<bool> { self.origin().is_enabled() }

  pub fn type_(&self) -> Option<TGProxyType> {
    self.origin().type_().clone()
      .map(|v| TGProxyType::of(&v))
      .map_or(None, |v| v)
  }

}

/// Contains information about a proxy server.
pub enum TGProxyType {
  /// A SOCKS5 proxy server.
  Socks5(Option<String>, Option<String>),
  /// A HTTP transparent proxy server.
  Http(Option<String>, Option<String>, Option<bool>),
  /// An MTProto proxy server.
  Mtproto(Option<String>),
}


impl TGProxyType {
  fn of(pxy_type: &Box<td_types::ProxyType>) -> Option<Self> {
    match td_types::RTDProxyTypeType::of(pxy_type.td_name()) {
      Some(td_types::RTDProxyTypeType::ProxyTypeSocks5) => {
        td_types::ProxyTypeSocks5::from_json(pxy_type.to_json()).map(|val| {
          TGProxyType::Socks5(val.username().clone(), val.password().clone())
        })
      }
      Some(td_types::RTDProxyTypeType::ProxyTypeHttp) => {
        td_types::ProxyTypeHttp::from_json(pxy_type.to_json()).map(|val| {
          TGProxyType::Http(val.username().clone(), val.password().clone(), val.http_only().clone())
        })
      }
      Some(td_types::RTDProxyTypeType::ProxyTypeMtproto) => {
        td_types::ProxyTypeMtproto::from_json(pxy_type.to_json()).map(|val| {
          TGProxyType::Mtproto(val.secret().clone())
        })
      }
      None => None
    }
  }
}


