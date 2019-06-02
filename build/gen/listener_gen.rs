use std::collections::HashMap;
use std::fs;
use std::path::Path;

use rstring_builder::StringBuilder;
use tera::{Context, Tera};
use text_reader::TextReader;

use crate::boml::lima;
use crate::boml::lima::Lima;



#[derive(Debug, Serialize)]
struct Ttmval {
  name: String,
  comment: Option<String>,
  tts: Vec<String>,
  tts_len: usize,
}


pub fn gen_listener() {
  let out_file = toolkit::path::root_dir().join("src/listener.rs");
  if out_file.exists() {
    fs::remove_file(out_file);
  }

  let tpl_path = toolkit::path::root_dir().join("build/tpl");
  let tera = Tera::new("build/tpl/**/*").expect("Can not create Tera template engine.");

  let mut context = Context::new();
  self::lima_data(&tpl_path, &mut context);
  let rscode = tera.render("listener_rs.tpl.txt", &context).expect("Can not render listener code.");
  toolkit::fs::append(out_file, rscode).expect("Write listener.rs fail.");
}


fn lima_data<S: AsRef<Path>>(tpl_path: S, context: &mut Context) {
  let limatoml = fs::read_to_string(tpl_path.as_ref().join("listener_rs.tpl.toml")).expect("Can not read mapper file");
  let lima = Lima::new(limatoml);
  let lin = lima.lin();
  let inf = lima.inf();

  let ttms = lin.names().iter()
    .map(|name| {
      let tts = lin.tts(name);
      let tts_len = tts.len();
      let ttmval = Ttmval {
        name: name.clone(),
        comment: lin.comment(name).map(|comment| lima::format_comment(comment, true)),
        tts,
        tts_len,
      };
      ttmval
    })
    .collect::<Vec<Ttmval>>();


  context.insert("ttms", &ttms);
  context.insert("mappers", &lin.mappers());
  context.insert("imports", &inf.imports());
  context.insert("comment_listener", &inf.comment_listener().map(|comment| lima::format_comment(comment, false)));
  context.insert("comment_lout", &inf.comment_lout().map(|comment| lima::format_comment(comment, false)));
}

