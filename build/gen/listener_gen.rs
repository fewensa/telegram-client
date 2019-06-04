use std::collections::HashMap;
use std::fs;
use std::path::Path;

use rstring_builder::StringBuilder;
use tera::{Context, Tera};

use crate::boml::lima;
use crate::boml::lima::{Lima, Ltt};

#[derive(Debug, Serialize)]
struct Ttmval {
  name: String,
  td_type: Option<String>,
  tg_struct: Option<String>,
  comment: Option<String>,
  tt: Option<Ltt>,
}


pub fn gen_listener() {
  let tpl_path = toolkit::path::root_dir().join("build/tpl");
  let tera = Tera::new("build/tpl/**/*").expect("Can not create Tera template engine.");

  let mut context = Context::new();
  self::lima_data(&tpl_path, &mut context);

  gen_listener_rs(&tera, &context);
  gen_handler_receive_rs(&tera, &context);
}

fn gen_listener_rs(tera: &Tera, context: &Context) {
  let out_file = toolkit::path::root_dir().join("src/listener.rs");
  if out_file.exists() {
    fs::remove_file(&out_file);
  }

  let rscode = tera.render("listener_rs.tpl.txt", &context).expect("Can not render listener code.");
  toolkit::fs::append(&out_file, rscode).expect("Write listener.rs fail.");
}

fn gen_handler_receive_rs(tera: &Tera, context: &Context) {
  let out_file = toolkit::path::root_dir().join("src/handler/handler_receive.rs");
  if out_file.exists() {
    fs::remove_file(&out_file);
  }

  let rscode = tera.render("handler_receive.rs.tpl.txt", &context).expect("Can not render handler_receive code.");
  toolkit::fs::append(&out_file, rscode).expect("Write handler_receive.rs fail.");
}


fn lima_data<S: AsRef<Path>>(tpl_path: S, context: &mut Context) {
  let limatoml = fs::read_to_string(tpl_path.as_ref().join("listener_rs.tpl.toml")).expect("Can not read mapper file");
  let lima = Lima::new(limatoml);
  let lin = lima.lin();
  let info = lima.info();

  let ttms = lin.names().iter()
    .map(|name| {
      let td_type = lin.td_type(name);
      let ttmval = Ttmval {
        name: name.clone(),
        td_type: td_type.clone(),
        tg_struct: td_type.map(|value| toolkit::text::uppercase_first_char(value)),
        comment: lin.comment(name).map(|comment| lima::format_comment(comment, true)),
        tt: lin.tt(name),
      };
      ttmval
    })
    .collect::<Vec<Ttmval>>();


  context.insert("ttms", &ttms);
  context.insert("td_types", &lin.td_types());
  context.insert("uses", &info.uses());
  context.insert("comment_listener", &info.comment_listener().map(|comment| lima::format_comment(comment, false)));
  context.insert("comment_lout", &info.comment_lout().map(|comment| lima::format_comment(comment, false)));
}

