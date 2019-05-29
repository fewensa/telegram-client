use std::collections::HashMap;
use std::fs;
use std::path::Path;

use rstring_builder::StringBuilder;
use tera::{Context, Tera};
use text_reader::TextReader;

pub fn gen_listener() {
  let _p = &format!("{}/src/listener.rs", toolkit::path::root_dir())[..];
  let out_file = Path::new(_p);
  if out_file.exists() {
    fs::remove_file(out_file);
  }

  let tpl_path = &format!("{}/build/tpl", toolkit::path::root_dir())[..];
  let tera = Tera::new(&format!("{}/**/*", tpl_path)).expect("Can not create Tera template engine.");

  let cfgt = fs::read_to_string(Path::new(tpl_path).join("listener_mapper.tpl.txt")).expect("Can not read mapper file");
  let mapper = self::ana_mapper(cfgt);

  let mut context = Context::new();
  context.insert("mapper", &mapper);
  let rscode = tera.render("listener_rs.tpl.txt", &context).expect("Can not render listener code.");

  toolkit::fs::append(out_file, rscode).expect("Write listener.rs fail.");
}

fn ana_mapper(text: String) -> HashMap<String, String> {
  let mut mapper = HashMap::new();
  let mut builder = StringBuilder::new();
  let mut reader = TextReader::new(text);

  let mut key = "".to_string();
  let mut value = "".to_string();
  let mut heqix = 0;
  while reader.has_next() {
    let this_line = reader.this_line();
    if this_line.is_none() {
      continue;
    }
    let this_line = this_line.unwrap();
    if this_line.is_empty() || "\n" == &this_line[..] {
      continue;
    }
    match reader.next() {
      Some('#') => {
        if key.is_empty() {
          continue;
        }
        if heqix == 0 {
          mapper.insert(key.clone().trim().to_string(), "".to_string());
          key = "".to_string();
          builder.clear();
          continue;
        }
        value = builder.string();
        mapper.insert(key.clone().trim().to_string(), value.clone().trim().to_string());
        key = "".to_string();
        value = "".to_string();
        heqix = 0;
        builder.clear();

        while reader.has_next() {
          match reader.next() {
            Some('\n') => break,
            _ => continue
          }
        }
      }
      Some('=') => {
        if heqix != 0 {
          panic!("Data fail");
        }
        if builder.is_empty() {
          panic!("Not have key");
        }
        key = builder.string();
        builder.clear();
        heqix = 1;
        continue
      }
      Some('\n') => {
        value = builder.string();
        if key.is_empty() {
          continue
        }
        mapper.insert(key.clone().trim().to_string(), value.clone().trim().to_string());
        key = "".to_string();
        value = "".to_string();
        heqix = 0;
        builder.clear();
        continue
      },
      Some(ch) => {
        builder.append(ch);
        continue
      },
      None => {}
    }
  };
  mapper
}


