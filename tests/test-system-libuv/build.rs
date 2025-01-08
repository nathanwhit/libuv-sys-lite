fn main() {
  if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
    if let Ok(link_path) = std::env::var("LIBUV_LIB_PATH") {
      for path in link_path.split(';') {
        println!("cargo:rustc-link-search={}", path);
      }
      println!("cargo:rustc-link-lib=uv");
      if let Ok(include_path) = std::env::var("LIBUV_INCLUDE_PATH") {
        println!("cargo:include={}", include_path);
      }
    } else {
      system_deps::Config::new().probe().unwrap();
    }
  } else {
    system_deps::Config::new().probe().unwrap();
  }
}
