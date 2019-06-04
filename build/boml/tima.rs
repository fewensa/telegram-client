#[derive(Debug)]
pub struct Tima {
  toml: toml::Value,
  tmod: Tmod,
  tgypes: Tgypes,
}

#[derive(Debug)]
pub struct Tmod {
  toml: toml::Value,
}

#[derive(Debug)]
pub struct Tgypes {
  toml: toml::Value
}

impl Tima {
  pub fn new(toml_text: String) -> Self {
    let toml: toml::Value = toml_text.parse().expect(&format!("Listener toml config format fail => {}", toml_text)[..]);
    Self {
      toml: toml.clone(),
      tmod: Tmod::new(toml.clone()),
      tgypes: Tgypes::new(toml.clone()),
    }
  }

  pub fn tmod(&self) -> &Tmod {
    &self.tmod
  }

  pub fn tgypes(&self) -> &Tgypes {
    &self.tgypes
  }
}


impl Tmod {
  pub fn new(toml: toml::Value) -> Self {
    let atoml = toml.get("tmod").expect(&format!("Listener config lose [inf] => {:?}", toml)[..]);
    if !atoml.is_table() {
      panic!("tmod is not a table  => {:?}", toml);
    }
    Self { toml: atoml.clone() }
  }

  fn array_string(&self, name: &'static str) -> Vec<String> {
    self.toml.get(name)
      .filter(|&value| value.is_array())
      .map(|value| value.as_array().unwrap())
      .map(|value| value.iter()
        .filter(|&item| item.is_str() && !item.as_str().unwrap().is_empty())
        .map(|item| item.as_str().unwrap().to_string())
        .collect::<Vec<String>>()
      )
      .map_or(vec![], |v| v)
  }

  pub fn def_use(&self) -> Vec<String> {
    self.array_string("def_use")
  }

  pub fn def_mod(&self) -> Vec<String> {
    self.array_string("def_mod")
  }
}

impl Tgypes {
  pub fn new(toml: toml::Value) -> Self {
    let atoml = toml.get("tgypes").expect(&format!("Listener config lose [inf] => {:?}", toml)[..]);
    if !atoml.is_table() {
      panic!("tgypes is not a table  => {:?}", toml);
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

  pub fn uses<S: AsRef<str>>(&self, name: S) -> Vec<String> {
    self.toml.get(name.as_ref())
      .filter(|&value| value.is_table())
      .map(|value| value.as_table().unwrap().get("uses")
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

  pub fn typen<S: AsRef<str>>(&self, name: S) -> String {
    self.string(name, "typen").expect(&format!("Lose typen => {:?}", self.toml)[..])
  }

  pub fn inner<S: AsRef<str>>(&self, name: S) -> String {
    self.string(name, "inner").expect(&format!("Lose inner => {:?}", self.toml)[..])
  }

  pub fn comment<S: AsRef<str>>(&self, name: S) -> Option<String> {
    self.string(name, "comment")
  }
}



