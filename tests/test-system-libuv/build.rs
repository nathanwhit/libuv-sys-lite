fn main() {
  if cfg!(target_os = "windows") {
    if let Ok(link_path) = std::env::var("LIBUV_LIB_PATH") {
      println!("cargo:rustc-link-search=native={}", link_path);
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
