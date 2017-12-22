extern crate libc;
#[macro_use]
extern crate emacs_module_bindings as emacs;

use emacs::{EmacsVal, EmacsRT, EmacsEnv, ConvResult};
use emacs::native2elisp as n2e;
use emacs::elisp2native as e2n;
use std::os::raw;
use std::ptr;
use std::ffi::CString;

/// This states that the module is GPL-compliant.
/// Emacs won't load the module if this symbol is undefined.
#[no_mangle]
#[allow(non_upper_case_globals)]
pub static plugin_is_GPL_compatible: libc::c_int = 0;

const MODULE: &str = "example-rs-module";

fn inc(env: *mut EmacsEnv, num: *mut EmacsVal) -> ConvResult<EmacsVal> {
    let i = e2n::integer(env, num, 0)?;
    n2e::integer(env, i + 1)
}

emacs_subrs!(
    f_inc(env, _nargs, args, _data, tag) {
        message!(env, "{}: {:?}", tag, args)?;
        inc(env, args)
    };
);

/// Entry point for live-reloading during development.
#[no_mangle]
pub extern "C" fn emacs_rs_module_init(env: *mut EmacsEnv) -> libc::c_int {
    message!(env, "Hello, Emacs!").unwrap();

    let doc = CString::new("This is a unicode doc string, from Nguyễn Tuấn Anh!").unwrap();
    emacs::bind_function(
        env, format!("{}/inc", MODULE),
        n2e::function(
            env, 1, 1, Some(f_inc),
            doc.as_ptr(), ptr::null_mut(),
        ).unwrap(),
    );

    emacs::provide(env, MODULE.to_string());
    0
}

/// Entry point for Emacs' loader, for "production".
#[no_mangle]
pub extern "C" fn emacs_module_init(ert: *mut EmacsRT) -> libc::c_int {
    let env = emacs::get_environment(ert);
    emacs_rs_module_init(env)
}
