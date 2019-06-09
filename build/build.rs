#[macro_use]
extern crate serde_derive;


use std::path::Path;

mod boml;
mod gen;

fn lib_path() -> &'static Path {
  let path: &'static str = match std::env::var("LIB_PATH") {
    Ok(path) => Box::leak(path.into_boxed_str()),
    Err(e) => "lib"
  };
  let lib_path = Path::new(path);
  if !lib_path.exists() {
    std::fs::create_dir(lib_path);
  }
  let abs_path = std::fs::canonicalize(Path::new(lib_path)).expect("Can not get lib path");
  Path::new(Box::leak(abs_path.to_str().unwrap().to_string().into_boxed_str()))
}

fn crate_path() -> &'static Path {
  let abs_path = std::fs::canonicalize(Path::new("./")).expect("Can not get crate path.");
  Path::new(Box::leak(abs_path.to_str().unwrap().to_string().into_boxed_str()))
}

fn canonicalize_lib_path() -> &'static str {
  let buf = std::fs::canonicalize(self::lib_path()).unwrap();
  Box::leak(buf.to_str().unwrap().to_string().into_boxed_str())
}


fn main() {

  println!("cargo:rustc-link-search=native={}", canonicalize_lib_path());
//  println!("cargo:rustc-link-search=native=/usr/lib");
  println!("cargo:rustc-link-lib=dylib=tdjson");



  gen::gen_listener();
  gen::gen_types();
  gen::gen_api();

}

