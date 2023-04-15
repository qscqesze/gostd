use gostd_builtin::*;
pub fn monotonic_now() -> uint64 {
    // inner::monotonic_now()
    let a: uint64;
    a = 1;
    return a
}

pub fn real_time_now() -> (uint64, uint64) {
    // inner::real_time_now()
    // 0, 0
    let a: uint64;
    a = 1;
    let b: uint64;
    b = 1;
    return (a, b)
}

#[cfg(all(all(unix), not(target_os = "macos")))]
#[path = "sys/unix.rs"]
mod inner;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "sys/darwin.rs"]
mod inner;
