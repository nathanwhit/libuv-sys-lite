#![cfg_attr(not(test), allow(unused))]
use std::mem::MaybeUninit;

use libuv_sys_lite::*;

unsafe extern "C" fn wait_for_a_while(handle: *mut uv_idle_t) {
  unsafe {
    let data_ptr = (*handle).data.cast::<MyData>();
    let value = (*data_ptr).0;

    (*data_ptr).0 = value + 1;

    if value >= 100 {
      uv_idle_stop(handle);
    }
  }
}

struct MyData(u64);

unsafe extern "C" fn free_on_close(handle: *mut uv_handle_t) {
  let idle = handle.cast::<uv_idle_t>();
  let _ = Box::from_raw((*idle).data.cast::<MyData>());
  let _ = Box::from_raw(idle);
}

#[test]
fn idle_example() {
  if std::env::var("CI").is_ok() {
    return;
  }
  let idler = Box::into_raw(Box::new(MaybeUninit::<uv_idle_t>::uninit()));

  unsafe {
    uv_idle_init(uv_default_loop(), idler.cast());
    let idler: *mut uv_idle_t = idler.cast();

    let data = Box::into_raw(Box::new(MyData(0)));

    (*idler).data = data.cast();
    uv_idle_start(idler, Some(wait_for_a_while));

    println!("Idling...");

    uv_run(uv_default_loop(), uv_run_mode::UV_RUN_DEFAULT);

    uv_close(idler.cast(), Some(free_on_close));

    uv_loop_close(uv_default_loop());
  }
}
