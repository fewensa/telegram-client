use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;

use case::CaseExt;
use rstring_builder::StringBuilder;
use tera::{Context, Tera};

use crate::boml::sima::*;
use crate::boml::aima::*;

pub fn gen_api() {
  let tpl_path = toolkit::path::root_dir().join("build/tpl");
  let tera = Tera::new("build/tpl/**/*").expect("Can not create Tera template engine.");

  let schema = toolkit::path::root_dir().join("schema/schema.toml");
  if schema.exists() {
    let sima_toml = fs::read_to_string(&schema).expect("Can not read mapper file");
    let aima_toml = fs::read_to_string(tpl_path.join("api_td_types.tpl.toml")).expect("Can not read mapper file");

    let sima = Sima::new(sima_toml);
    let aima = Aima::new(aima_toml);

    self::gen_api_rs(&tera, &sima);
    self::gen_tdfn(&tera, &sima);
    self::gen_aima(&tera, &sima, &aima);
  }
}

fn gen_api_rs(tera: &Tera, sima: &Sima) {
  let api_file = toolkit::path::root_dir().join("src/api/api.rs");
  if api_file.exists() {
    fs::remove_file(&api_file);
  }

  let mut context = Context::new();

  let afs = sima.all_functions();

  let classes = afs.iter().map(|v| {
    let mut map = HashMap::new();
    map.insert("class_name", v.to_string());
    map.insert("class_snake", (&v[..]).to_snake());
    map
  })
    .collect::<Vec<HashMap<&str, String>>>();
  context.insert("classes", &classes);

  let rscode = tera.render("api_rs.tpl.txt", &context).expect("Can not render api code.");
  toolkit::fs::append(&api_file, &rscode).expect("Can not write api.rs");
}


fn gen_tdfn(tera: &Tera, sima: &Sima) {
  let afs = sima.all_functions();

  let t_rs = toolkit::path::root_dir().join(&format!("src/api/function_builder.rs")[..]);
  if t_rs.exists() {
    fs::remove_file(&t_rs);
  }

  let tdrscommon = tera.render("api_td_rs_common.tpl.txt", &Context::new()).expect("Can not render tdlib function code.");
  toolkit::fs::append(&t_rs, &tdrscommon).expect(&format!("Can not write {:?}", t_rs)[..]);

  afs.iter().for_each(|v| {
    let clz = sima.clz(v).expect(&format!("Lost class for {}", v)[..]);
    let mut context = Context::new();
    context.insert("td", &clz);

    let rscode = tera.render("api_td_rs.tpl.txt", &context).expect("Can not render tdlib function code.");
    toolkit::fs::append(&t_rs, &rscode).expect(&format!("Can not write {:?}", t_rs)[..]);
  })
}


fn gen_aima(tera: &Tera, sima: &Sima, aima: &Aima) {
  let tmod = aima.tmod();
  let type_dir = Path::new("src/api");
  self::write_tmod(&type_dir, aima, tera);
  self::write_types(&type_dir, sima, aima, tera);
}




fn write_types<P: AsRef<Path>>(type_dir: P, sima: &Sima, aima: &Aima, tera: &Tera) {
  let tgypes = aima.tgypes();

  tgypes.names().iter().for_each(|name| {
    let t_rs = type_dir.as_ref().join(&format!("t_{}.rs", name)[..]);
    let f_rs = type_dir.as_ref().join(&format!("f_{}.rs", name)[..]);
    if t_rs.exists() {
      fs::remove_file(&t_rs);
    }
    if !f_rs.exists() {
      File::create(&f_rs).expect(&format!("Can not create [{:?}] file", f_rs)[..]);
    }

    let rscode = tera.render("api_td_types.use.tpl.txt", &Context::new()).expect("Can not render types code.");
    toolkit::fs::append(&t_rs, rscode).expect(&format!("Can not create [{:?}] file", t_rs)[..]);

    tgypes.td_types(name).iter().for_each(|td| {
      let mut context = Context::new();

      let clz = sima.clz(td.mapper.clone()).expect(&format!("Fail td mapper => {} => {}", td.typen.clone(), td.mapper.clone())[..]);
      context.insert("td", &clz);
      context.insert("tgype", &td);

      let rscode = tera.render("api_td_types.tpl.txt", &context).expect("Can not render types code.");
      toolkit::fs::append(&t_rs, rscode).expect(&format!("Can not create [{:?}] file", t_rs)[..]);
    });
  });
}





fn write_tmod<P: AsRef<Path>>(type_dir: P, aima: &Aima, tera: &Tera) {
  let tgypes = aima.tgypes();
  let tmod = aima.tmod();
  let mod_rs = type_dir.as_ref().join("mod.rs");
  if mod_rs.exists() {
    fs::remove_file(&mod_rs).expect(&format!("Can not create [{:?}]", &mod_rs)[..]);
  }

  let mut builder = StringBuilder::new();

  tmod.uses().iter().for_each(|item| { builder.append("pub use ").append(item.clone()).append(";\n"); });


  tgypes.names().iter().for_each(|name| {
    let t_rs = &format!("t_{}", name)[..];
    builder.append("pub use self::").append(t_rs).append("::*;\n");
  });
  builder.append("\n\n\n");


  tmod.mods().iter().for_each(|item| {
    builder.append(if item.macro_use { "#[macro_use] " } else { "" })
      .append("mod ")
      .append(item.name.clone()).append(";\n\n");
  });

  tgypes.names().iter().for_each(|name| {
    let t_rs = &format!("t_{}", name)[..];
    let f_rs = &format!("f_{}", name)[..];
    builder.append("mod ").append(t_rs).append(";\n");
    builder.append("mod ").append(f_rs).append(";\n");
  });

  toolkit::fs::append(&mod_rs, builder.string())
    .expect(&format!("Can not create [{:?}] file", mod_rs)[..]);
}



