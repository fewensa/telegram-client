use tera::Tera;

use crate::boml::sima::*;
use std::fs;

pub fn gen_api() {
  let tpl_path = toolkit::path::root_dir().join("build/tpl");
  let tera = Tera::new("build/tpl/**/*").expect("Can not create Tera template engine.");

  let toml_text = fs::read_to_string(toolkit::path::root_dir().join("schema/schema.toml")).expect("Can not read mapper file");


  let sima = Sima::new(toml_text);

}

