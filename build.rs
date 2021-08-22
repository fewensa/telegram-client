
fn main() {
  println!("cargo:rustc-link-search=native=D:/dev/telegram/.maintain/dylib/tags/v1.7.0");
  println!("cargo:rustc-link-lib=dylib=tdjson");
}

