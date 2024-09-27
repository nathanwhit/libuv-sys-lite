use std::path::PathBuf;

fn main() {
  let mut builder = bindgen::builder().header("libuv-1.48.0-include/uv.h");
  builder = builder
    .allowlist_type("uv_.+")
    .allowlist_type("UV_.+")
    .allowlist_var("uv_.+")
    .allowlist_var("UV_.+")
    .allowlist_type("FILE")
    .default_enum_style(bindgen::EnumVariation::NewType {
      is_bitfield: false,
      is_global: false,
    });
  builder
    .clang_arg("-Ilibuv-1.48.0-include")
    .generate()
    .unwrap()
    .write_to_file(PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"))
    .unwrap();
}
