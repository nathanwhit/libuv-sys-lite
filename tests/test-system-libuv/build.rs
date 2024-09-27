fn main() {
  if std::env::var("CI").is_err() {
    system_deps::Config::new().probe().unwrap();
  }
}
