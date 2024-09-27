fn main() {
  if std::env::var("RUN_TEST").is_ok() {
    system_deps::Config::new().probe().unwrap();
  }
}
