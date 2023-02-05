// SPDX-License-Identifier: Apache-2.0

macro_rules! test {
    () => (cfg!(test) && ::std::env::var("_CLANG_SYS_TEST").is_ok());
}

macro_rules! os {
    ($os:expr) => {
        if cfg!(test) && ::std::env::var("_CLANG_SYS_TEST").is_ok() {
            let var = ::std::env::var("_CLANG_SYS_TEST_OS");
            var.map_or(false, |v| v == $os)
        } else {
            cfg!(target_os = $os)
        }
    };
}

macro_rules! pointer_width {
    ($pointer_width:expr) => {
        if cfg!(test) && ::std::env::var("_CLANG_SYS_TEST").is_ok() {
            let var = ::std::env::var("_CLANG_SYS_TEST_POINTER_WIDTH");
            var.map_or(false, |v| v == $pointer_width)
        } else {
            cfg!(target_pointer_width = $pointer_width)
        }
    };
}
