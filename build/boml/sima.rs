use crate::boml::lima;

pub struct Sima {
  toml: toml::Value
}


#[derive(Debug, Clone, Serialize)]
pub struct TdClz {
  origin_name: String,
  clz_name: String,
  clz_is_trait: bool,
  clz_description: Option<String>,
  fields: Vec<TdField>,
  has_trait_field: bool,
  derives: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TdField {
  doc_hidden: bool,
  name: String,
  class: String,
  class_real: String,
  description: Option<String>,
  tags: Vec<String>,
  native: bool,
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

  pub fn clz<S: AsRef<str>>(&self, name: S) -> Option<TdClz> {
    let name = name.as_ref();
    self.td().get(name)
      .filter(|&v| v.is_table())
      .map(|v| {
        let origin_name = v.get("origin_name").expect(&format!("Lost origin_name => {}", name)[..]).as_str().expect(&format!("origin_name fail => {}", name)[..]);
        let clz_name = v.get("clz_name").expect(&format!("Lost clz_name => {}", name)[..]).as_str().expect(&format!("clz_name fail => {}", name)[..]);
        let clz_is_trait = v.get("clz_is_trait").map_or(false, |v| v.as_bool().map_or(false, |v| v));
        let clz_description = v.get("clz_description").map_or(None, |v| v.as_str().map_or(None, |v| Some(v)));
//          .map(|v| lima::format_comment(v, false));
        let has_trait_field = v.get("has_trait_field").map_or(false, |v| v.as_bool().map_or(false, |v| v));
        let derives = vec!["Debug", "Clone"];
        let fields = v.get("fields")
          .filter(|&v| v.is_array())
          .map(|v| v.as_array().unwrap())
          .map(|v| v.iter()
            .filter(|&v| v.is_table())
            .map(|v| {
              let doc_hidden = v.get("doc_hidden").map_or(false, |v| v.as_bool().map_or(false, |v| v));
              let name = v.get("name").expect(&format!("Lost name for {} field", name)[..]).as_str().expect(&format!("{} field name fail.", name)[..]);
              let class = v.get("class").expect(&format!("Lost class for {} field", name)[..]).as_str().expect(&format!("{} field class fail.", name)[..]);
              let description = v.get("description").map_or(None, |v| v.as_str().map_or(None, |v| Some(v)));
//                .map(|v| lima::format_comment(v, true));
              let tags = v.get("tags").map_or(vec![], |v| v.as_array().map_or(vec![], |v| v.iter()
                .filter(|&v| v.is_str())
                .map(|v| v.as_str().unwrap().to_string())
                .collect(),
              ));
              TdField {
                doc_hidden,
                name: name.to_string(),
                class: class.to_string(),
                class_real: self::real_class(&class),
                description: description.map(|v| v.to_string()),
                tags,
                native: self::is_native_field(&class)
              }
            })
            .collect::<Vec<TdField>>()
          )
          .map_or(vec![], |v| v);
        TdClz {
          origin_name: origin_name.to_string(),
          clz_name: clz_name.to_string(),
          clz_is_trait,
          clz_description: clz_description.map(|v| v.to_string()),
          fields,
          has_trait_field,
          derives: derives.iter().map(|&v| v.to_string()).collect(),
        }
      })
  }
}


fn real_class<S: AsRef<str>>(class: S) -> String {
  let class = class.as_ref();
  if !class.starts_with("Option") {
    return class.to_string();
  }
  let class = class.replace("Option<", "");
  let class = &class[..class.len() - 1];
  class.to_string()
}

fn is_native_field<S: AsRef<str>>(class: S) -> bool {
  let class = self::real_class(class.as_ref()).to_lowercase();
  match &class[..] {
    "string" => true,
    "i32" | "i64" | "f32" | "f64" => true,
    "vec<i32>" | "vec<i64>" | "vec<f32>" | "vec<f64>" => true,
    "bool" => true,
    _ => false
  }
}


