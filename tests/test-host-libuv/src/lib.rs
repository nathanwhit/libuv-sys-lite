#![allow(clippy::missing_safety_doc)]
use std::{
  mem::MaybeUninit,
  sync::atomic::{AtomicBool, Ordering::SeqCst},
};

use libuv_sys_lite::{uv_handle_t, uv_idle_t, uv_loop_t};

struct Called {
  loop_: AtomicBool,
  idle: AtomicBool,
  close: AtomicBool,
}

static CALLED: Called = Called {
  loop_: AtomicBool::new(false),
  idle: AtomicBool::new(false),
  close: AtomicBool::new(false),
};

#[no_mangle]
pub unsafe extern "C" fn uv_default_loop() -> *mut uv_loop_t {
  CALLED.loop_.store(true, SeqCst);
  Box::into_raw(Box::new(MaybeUninit::<uv_loop_t>::uninit())).cast()
}

#[no_mangle]
pub unsafe extern "C" fn uv_idle_init(_loop_: *mut uv_loop_t, _idle: *mut uv_idle_t) {
  CALLED.idle.store(true, SeqCst);
}

#[no_mangle]
pub unsafe extern "C" fn uv_close(_handle: *mut uv_handle_t) {
  CALLED.close.store(true, SeqCst);
}

#[test]
fn idle_example() {
  eprintln!(
    "{:?}",
    std::process::Command::new("cargo")
      .arg("build")
      .arg("-p")
      .arg("libuv-dylib")
      .output()
      .unwrap()
  );
  // escargot::CargoBuild::new()
  //   .manifest_path("../../test-host-libuv/Cargo.toml")
  //   .package("test-host-libuv")
  //   .exec()
  //   .unwrap();
  let lo =
    unsafe { libloading::Library::new(libloading::library_filename("libuv_dylib")).unwrap() };
  let doit = unsafe { lo.get::<unsafe extern "C" fn()>(c"doit".to_bytes_with_nul()) };

  let doit = doit.unwrap();
  unsafe {
    doit();
  }

  assert!(CALLED.loop_.load(SeqCst));
  assert!(CALLED.idle.load(SeqCst));
  assert!(CALLED.close.load(SeqCst));
}
