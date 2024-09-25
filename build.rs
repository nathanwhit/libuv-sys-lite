use std::path::PathBuf;

fn main() {
    let mut builder = bindgen::builder().header("libuv-1.48.0-include/uv.h");
    for file in std::fs::read_dir("libuv-1.48.0-include/uv").unwrap() {
        let file = file.unwrap();
        let path = file.path();
        if path.extension().map(|ext| ext == "h").unwrap_or(false) {
            builder = builder.allowlist_file(path.to_str().unwrap());
        }
    }
    builder = builder.allowlist_item("uv_.+").allowlist_item("UV_.+");
    builder
        .clang_arg("-Ilibuv-1.48.0-include")
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();
}
