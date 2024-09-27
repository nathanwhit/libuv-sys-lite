use std::env;
fn main() {
  // taken from deno https://github.com/denoland/deno/blob/main/cli/build.rs
  let symbols_file_name = match env::consts::OS {
    "android" | "freebsd" | "openbsd" => "generated_symbol_exports_list_linux.def".to_string(),
    os => format!("generated_symbol_exports_list_{}.def", os),
  };
  let symbols_path = std::path::Path::new(".")
    .join(symbols_file_name)
    .canonicalize()
    .expect("Missing symbols list! Generate using generate_symbols_lists.js");

  println!("cargo:rustc-rerun-if-changed={}", symbols_path.display());

  #[cfg(target_os = "windows")]
  println!("cargo:rustc-link-arg=/DEF:{}", symbols_path.display());

  #[cfg(target_os = "macos")]
  println!(
    "cargo:rustc-link-arg=-Wl,-exported_symbols_list,{}",
    symbols_path.display()
  );

  #[cfg(target_os = "linux")]
  {
    println!(
      "cargo:rustc-link-arg=-Wl,--export-dynamic-symbol-list={}",
      symbols_path.display()
    );
  }

  #[cfg(target_os = "android")]
  println!(
    "cargo:rustc-link-arg=-Wl,--export-dynamic-symbol-list={}",
    symbols_path.display()
  );
}
