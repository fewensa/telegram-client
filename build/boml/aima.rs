
use crate::boml::lima;


#[derive(Debug)]
pub struct Aima {
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

#[derive(Debug, Clone, Serialize)]
pub struct GTdType {
  pub uses: Vec<String>,
  pub typen: String,
  pub inner: String,
  pub already_define: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DefMod {
  pub name: String,
  pub macro_use: bool
}

impl Aima {
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

  pub fn uses(&self) -> Vec<String> {
    self.array_string("uses")
  }

  pub fn mods(&self) -> Vec<DefMod> {
    self.toml.get("mods")
      .filter(|&value| value.is_array())
      .map(|value| value.as_array().unwrap())
      .map(|v| v.iter()
        .filter(|&v| v.is_table())
        .map(|v| v.as_table().unwrap())
        .filter(|&v| {
          let name = v.get("name");
          if name.is_none() { return false }
          if !name.unwrap().is_str() { return false }
          let mu = v.get("macro_use");
          if mu.is_some() {
            if !mu.unwrap().is_bool() { return false }
          }
          true
        })
        .map(|v| {
          let name = v.get("name").unwrap().as_str().unwrap().to_string();
          let mu = v.get("macro_use").map(|v| v.as_bool().unwrap()).map_or(false, |v| v);
          DefMod { name, macro_use: mu }
        })
        .collect::<Vec<DefMod>>()
      )
      .map_or(vec![], |v| v)
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

  pub fn td_types<S: AsRef<str>>(&self, name: S) -> Vec<GTdType> {
    self.toml.get(name.as_ref())
      .filter(|&v| v.is_array())
      .map(|v| v.as_array().unwrap())
      .filter(|&v| !v.is_empty())
      .map(|v| v.iter()
        .filter(|&v| {
          let uses = v.get("uses");
          let typen = v.get("typen");
          let inner = v.get("inner");
          if inner.is_none() { return false; }
          if !inner.unwrap().is_str() { return false; }
          if inner.unwrap().as_str().unwrap().is_empty() { return false; }
          if uses.is_some() {
            if !uses.unwrap().is_array() { return false; }
          }
          if typen.is_some() {
            if !typen.unwrap().is_str() { return false }
          }
          true
        })
        .map(|v| {
          let inner = v.get("inner").unwrap().as_str().map(|v| v.to_string()).unwrap();
          let typen = v.get("typen")
            .filter(|&v| v.is_str())
            .map(|v| v.as_str().unwrap())
            .filter(|&v| !v.is_empty())
            .map(|v| v.to_string())
            .map_or(format!("TG{}", inner), |v| v);
          let already_define = v.get("already_define").filter(|&v| v.is_bool()).map(|v| v.as_bool().unwrap()).map_or(false, |v| v);
          GTdType {
            uses: v.get("uses")
              .map(|v| v.as_array())
              .filter(|&v| v.is_some())
              .map(|v| v.unwrap())
              .map(|v| v.iter()
                .filter(|&v| v.is_str())
                .map(|v| v.as_str())
                .filter(|v| v.is_some())
                .map(|v| v.unwrap().to_string())
                .collect::<Vec<String>>()
              ).map_or(vec![], |v| v),
            typen,
            inner: inner,
            already_define
          }
        })
        .collect::<Vec<GTdType>>()
      )
      .map_or(vec![], |v| v)
  }


}



