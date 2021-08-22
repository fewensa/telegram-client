use std::path::Path;

use rtdlib::types::*;

#[derive(Debug, Clone)]
pub struct Config {
  toml: toml::Value
}

impl Default for Config {
  fn default() -> Self {
    let toml_file = match hostname::get_hostname() {
      Some(name) => format!("telegram-client.{}.toml", name),
      None => "telegram-client.toml".to_string()
    };
    let mut toml_file = Path::new("conf").join(&toml_file[..]);
    if !toml_file.exists() {
      toml_file = toolkit::path::root_dir().join("conf").join("telegram-client.toml");
    }
    if !toml_file.exists() {
      panic!("Not found config file");
    }
    debug!("Use {:?} config file", toml_file);
    let toml = std::fs::read_to_string(toml_file).unwrap();
    Config::new(toml)
  }
}

impl Config {
  fn new<S: AsRef<str>>(toml: S) -> Self {
    let value: toml::Value = toml::from_str(toml.as_ref()).unwrap();
    Self { toml: value }
  }

  pub fn proxy(&self) -> Option<AddProxy> {
    self.toml.get("proxy")
      .filter(|&v| v.is_table())
      .map(|v| v.as_table())
      .filter(|v| v.is_some())
      .map(|v| v.unwrap())
      .map(|v| {
        let server = v.get("server").unwrap().as_str().unwrap();
        let port = v.get("port").unwrap().as_integer().unwrap();
        let enable = v.get("enable").unwrap().as_bool().unwrap();
        let type_ = v.get("type").unwrap().as_str().unwrap();
        let mut tga = AddProxy::builder();
        tga.server(server)
          .port(port as i64)
          .enable(enable);
        match type_ {
          "socks5" => tga.type_(ProxyType::socks5(ProxyTypeSocks5::builder())),
          "http" => tga.type_(ProxyType::http(ProxyTypeHttp::builder())),
          "mtproto" => tga.type_(ProxyType::mtproto(ProxyTypeMtproto::builder())),
          _ => panic!("Not found proxy type")
        };
        tga.build()
      })
  }

  pub fn log(&self) -> Option<Log> {
    self.toml.get("log")
      .filter(|&v| v.is_table())
      .map(|v| v.as_table())
      .filter(|&v| v.is_some())
      .map(|v| v.unwrap())
      .map(|v| {
        let type_ = match v.get("type").unwrap().as_str().unwrap() {
          "console" => LogType::Console,
          "file" => LogType::File,
          _ => LogType::Console
        };
        let path = v.get("path").filter(|&v| v.is_str())
          .map(|v| v.as_str())
          .filter(|&v| v.is_some())
          .map(|v| v.unwrap().to_string());
        let level = v.get("level").filter(|&v| v.is_integer())
          .map(|v| v.as_integer())
          .map(|v| v.unwrap())
          .map_or(1, |v| v);
        Log { type_, path, level }
      })
  }
}

#[derive(Debug, Clone)]
pub struct Log {
  pub type_: LogType,
  pub path: Option<String>,
  pub level: i64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum LogType {
  Console,
  File,
}




