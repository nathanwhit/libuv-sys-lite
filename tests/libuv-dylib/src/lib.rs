use std::{mem::MaybeUninit, sync::Once};

use libuv_sys_lite::*;

static SETUP: Once = Once::new();

#[no_mangle]
extern "C" fn doit() {
  SETUP.call_once(|| unsafe {
    libuv_sys_lite::setup();
  });
  unsafe {
    let loop_ = uv_default_loop();
    let idler = Box::into_raw(Box::new(MaybeUninit::<uv_idle_t>::uninit()));
    uv_idle_init(loop_, idler.cast());
    uv_close(idler.cast(), None);
  }
}
