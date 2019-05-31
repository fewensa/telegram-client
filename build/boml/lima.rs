use std::fs;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Lima {
  toml: toml::Value,
  lin: Lin,
  inf: Inf,
}

#[derive(Debug)]
pub struct Lin {
  toml: toml::Value
}

#[derive(Debug)]
pub struct Inf {
  toml: toml::Value
}

impl Lima {
  pub fn new(toml_text: String) -> Self {
    let toml: toml::Value = toml_text.parse().expect(&format!("Listener toml config format fail => {}", toml_text)[..]);
    Self {
      toml: toml.clone(),
      lin: Lin::new(toml.clone()),
      inf: Inf::new(toml.clone()),
    }
  }

  pub fn lin(&self) -> &Lin {
    &self.lin
  }

  pub fn inf(&self) -> &Inf {
    &self.inf
  }
}

impl Lin {
  pub fn new(toml: toml::Value) -> Self {
    let atoml = toml.get("lin").expect(&format!("Listener config lose [lin] => {:?}", toml)[..]);
    if !atoml.is_table() {
      panic!("lin is not a table  => {:?}", toml);
    }
    Self { toml: atoml.clone() }
  }

  pub fn names(&self) -> Vec<String> {
    self.toml.as_table().map(|table| table.iter().map(|(key, _value)| key.clone()).collect::<Vec<String>>())
      .map_or(vec![], |v| v)
  }

  fn string<N: AsRef<str>, A: AsRef<str>>(&self, name: N, attr: A) -> Option<String> {
    self.toml.get(name.as_ref())
      .filter(|&value| value.is_table())
      .map(|value| value.as_table()
        .unwrap()
        .get(attr.as_ref())
        .filter(|&value| value.is_str())
        .map(|value| value.as_str())
      )
      .map_or(None, |v| v.filter(|v| v.is_some() && !v.unwrap().is_empty())
        .map(|v| v.unwrap().to_string()))
  }

  pub fn mappers(&self) -> Vec<String> {
    self.names().iter()
      .map(|name| self.mapper(name))
      .filter(|item| item.is_some())
      .map(|item| item.unwrap())
      .filter(|item| !item.is_empty())
      .collect()
  }

  pub fn mapper<S: AsRef<str>>(&self, name: S) -> Option<String> {
    self.string(name, "mapper")
  }

  pub fn tts<S: AsRef<str>>(&self, name: S) -> Vec<String> {
    self.toml.get(name.as_ref())
      .filter(|&value| value.is_table())
      .map(|value| value.as_table().unwrap().get("tt")
        .filter(|&value| value.is_array())
        .map(|value| value.as_array().unwrap())
        .map(|value| {
          value.iter()
            .filter(|&value| value.is_str() && !value.as_str().unwrap().is_empty())
            .map(|value| value.as_str().unwrap().to_string())
            .collect::<Vec<String>>()
        })
        .map_or(vec![], |value| value)
      )
      .map_or(vec![], |value| value)

  }

  pub fn comment<S: AsRef<str>>(&self, name: S) -> Option<String> {
    self.string(name, "comment")
  }

}

impl Inf {
  pub fn new(toml: toml::Value) -> Self {
    let atoml = toml.get("inf").expect(&format!("Listener config lose [inf] => {:?}", toml)[..]);
    if !atoml.is_table() {
      panic!("inf is not a table  => {:?}", toml);
    }
    Self { toml: atoml.clone() }
  }

  pub fn imports(&self) -> Vec<String> {
    self.toml.get("imports")
      .filter(|value| value.is_array())
      .map(|value| value.as_array())
      .filter(|value| value.is_some())
      .map(|value| value.unwrap())
      .map(|values| values.iter()
        .filter(|&item| item.is_str())
        .map(|item| item.as_str().unwrap().to_string())
        .collect::<Vec<String>>()
      )
      .map_or(vec![], |v| v)
  }

  pub fn comment_listener(&self) -> Option<String> {
    self.toml.get("comment_listener")
      .filter(|value| value.is_str())
      .map(|value| value.as_str().unwrap().to_string())
  }

  pub fn comment_lout(&self) -> Option<String> {
    self.toml.get("comment_lout")
      .filter(|value| value.is_str())
      .map(|value| value.as_str().unwrap().to_string())
  }
}



pub fn format_comment<S: AsRef<str>>(comment: S, fill_space: bool) -> String {
  let comment = comment.as_ref();
  if comment.is_empty() {
    return comment.to_string();
  }
  let comment = comment.replace("\n", if fill_space { "\n  /// " } else { "\n/// " } );
  format!("/// {}", comment)
}
