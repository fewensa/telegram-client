


pub struct Sima {
  toml: toml::Value
}

impl Sima {
  pub fn new(toml_text: String) -> Self {
    let toml: toml::Value = toml_text.parse().expect(&format!("Schema toml config format fail => {}", toml_text)[..]);
    Self {
      toml: toml.clone()
    }
  }

  fn td(&self) -> &toml::Value {
    self.toml.get("rtd")
      .filter(|&v| v.is_table())
      .unwrap()
  }

  pub fn all_functions(&self) -> Vec<String> {
    self.td().get("Function")
      .filter(|&v| v.is_table())
      .map(|v| v.get("sub_classes")
        .filter(|&v| v.is_array())
        .map(|v| v.as_array())
        .filter(|&v| v.is_some())
        .map(|v| v.unwrap())
        .map(|v| v.iter()
          .filter(|&v| v.is_table())
          .filter(|&v| v.get("is_trait")
            .filter(|&v| v.is_bool())
            .map(|v| v.as_bool())
            .map(|v| v.map_or(false, |v| v))
            .map_or(false, |v| !v)
          )
          .map(|v| v.get("name")
            .filter(|&v| v.is_str())
            .map(|v| v.as_str())
            .filter(|&v| v.is_some())
            .map(|v| v.unwrap().to_string())
            .unwrap()
          )
          .collect::<Vec<String>>()
        )
        .map_or(vec![], |v| v)
      )
      .map_or(vec![], |v| v)
  }

}


