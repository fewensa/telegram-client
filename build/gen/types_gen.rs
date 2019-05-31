use std::path::Path;
use std::fs;
use tera::{Tera, Context};
use crate::boml::tima::Tima;
use std::fs::File;
use rstring_builder::StringBuilder;
use crate::boml::lima;

pub fn gen_types() {
  let _types_dir = Path::new(&toolkit::path::root_dir()[..]).join("src/types");
  if !_types_dir.exists() {
    fs::create_dir_all(&_types_dir);
  }
  let _types_dir = toolkit::path::canonicalize_path(_types_dir).unwrap();
  let type_dir = Path::new(&_types_dir[..]);

  let tpl_path = &format!("{}/build/tpl", toolkit::path::root_dir())[..];
  let tera = Tera::new(&format!("{}/**/*", tpl_path)).expect("Can not create Tera template engine.");

  let toml_text = fs::read_to_string(&format!("{}/tg_types.tpl.toml", tpl_path)[..]).expect("Not found tg types toml config file");
  let tima = Tima::new(toml_text);

  self::write_types(&type_dir, &tima, &tera);
  self::write_tmod(&type_dir, &tima, &tera);
}

fn write_types<P: AsRef<Path>>(type_dir: P, tima: &Tima, tera: &Tera) {
  let tgypes = tima.tgypes();

  tgypes.names().iter().for_each(|name| {
    let cap = toolkit::path::canonicalize_path(type_dir.as_ref()).unwrap();
    let _t_rs = format!("{}/t_{}.rs", cap, name);
    let _f_rs = format!("{}/f_{}.rs", cap, name);
    let t_rs = Path::new(&_t_rs[..]);
    let f_rs = Path::new(&_f_rs[..]);
    if t_rs.exists() {
      fs::remove_file(t_rs);
    }
    if !f_rs.exists() {
      File::create(f_rs).expect(&format!("Can not create [{}/f_{}.rs] file", cap, name)[..]);
    }

    let mut context = Context::new();
    context.insert("comment", &tgypes.comment(name).map(|comment| lima::format_comment(comment, false)));
    context.insert("typen", &tgypes.typen(name));
    context.insert("inner", &tgypes.inner(name));
    context.insert("imports", &tgypes.imports(name));

    let rscode = tera.render("tg_types.tpl.txt", &context).expect("Can not render types code.");
    toolkit::fs::append(t_rs, rscode).expect(&format!("Can not create [{}/t_{}.rs] file", cap, name)[..]);
  });
}


fn write_tmod<P: AsRef<Path>>(type_dir: P, tima: &Tima, tera: &Tera) {
  let tgypes = tima.tgypes();
  let tmod = tima.tmod();
  let mod_rs = type_dir.as_ref().join("mod.rs");
  if mod_rs.exists() {
    fs::remove_file(&mod_rs).expect(&format!("Can not create [{:?}]", &mod_rs)[..]);
  }

  let mut builder = StringBuilder::new();

  tmod.def_use().iter().for_each(|item| { builder.append("pub use ").append(item.clone()).append(";\n\n"); });


  tgypes.names().iter().for_each(|name| {
    let t_rs = &format!("t_{}", name)[..];
    builder.append("pub use self::").append(t_rs).append("::*;\n");
  });
  builder.append("\n\n\n");


  tmod.def_mod().iter().for_each(|item| { builder.append("mod ").append(item.clone()).append(";\n\n"); });

  tgypes.names().iter().for_each(|name| {
    let t_rs = &format!("t_{}", name)[..];
    let f_rs = &format!("f_{}", name)[..];
    builder.append("mod ").append(t_rs).append(";\n");
    builder.append("mod ").append(f_rs).append(";\n");
  });

  toolkit::fs::append(&mod_rs, builder.string()).expect(&format!("Can not create [{}] file", toolkit::path::canonicalize_path(mod_rs).unwrap())[..]);
}
