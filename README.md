# libuv-sys-lite

Tiny binding to libuv, that does not link to the library. The expectation is that
the consumer will link to the appropriate library.

Essentially just a rust version of `uv.h`, to provide interfaces without making decisions
on how to provide the actual implementation. This is similar to [`napi-sys`](https://github.com/napi-rs/napi-sys).
