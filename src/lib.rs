extern crate libc;
#[macro_use]
extern crate emacs_module_bindings as emacs;
extern crate libloading as lib;
#[macro_use]
extern crate lazy_static;

use emacs::{EmacsVal, EmacsRT, EmacsEnv, ConvResult};
use emacs::native2elisp as n2e;
use emacs::elisp2native as e2n;
use std::os::raw;
use std::ptr;
use std::ffi::CString;
use std::collections::HashMap;
use std::sync::Mutex;

/// This states that the module is GPL-compliant.
/// Emacs won't load the module if this symbol is undefined.
#[no_mangle]
#[allow(non_upper_case_globals)]
pub static plugin_is_GPL_compatible: libc::c_int = 0;

lazy_static! {
    static ref LIBRARIES: Mutex<HashMap<String, lib::Library>> = Mutex::new(HashMap::new());
}

const INIT_FROM_ENV: &str = "emacs_rs_module_init";
const RS_MODULE: &str = "rs-module";

/// Helper function that enables live-reloading of Emacs' dynamic module. To be reloadable, the
/// module be loaded by this function (`rs/module-load` in ELisp) instead of Emacs'
/// `module-load`. (Re)loading is achieved by calling `(rs/module-load "/path/to/module")`.
fn load_module(env: *mut EmacsEnv, path: String) -> ConvResult<EmacsVal> {
    let mut libraries = LIBRARIES.lock()
        .expect("Failed to acquire lock for module map");
    // TODO: How about tracking by feature name?
    match libraries.remove(&path) {
        Some(l) => message!(env, "[{}]: unloaded {:?}...", &path, &l)?,
        None => message!(env, "[{}]: not loaded yet", &path)?,
    };
    message!(env, "[{}]: loading...", &path)?;
    let l = lib::Library::new(&path)?;
    message!(env, "[{}]: initializing...", &path)?;
    unsafe {
        let rs_init: lib::Symbol<unsafe extern fn(*mut EmacsEnv) -> u32> =
            l.get(INIT_FROM_ENV.as_bytes())?;
        rs_init(env);
    }
    libraries.insert(path.clone(), l);
    message!(env, "[{}]: loaded and initialized", &path)
}

emacs_subrs!(
    f_load_module(env, _nargs, args, _data, _tag) {
        let path = e2n::string(env, *args.offset(0))?;
        load_module(env, path)
    };
);

/// This is not exported, since this module should be loaded by Emacs' built-in `module-load`, thus
/// cannot be reloaded.
fn load(env: *mut EmacsEnv) -> libc::c_int {
    let fn_name = format!("{}/load", RS_MODULE);
    message!(env, "[{}]: loading...", RS_MODULE).unwrap();
    // Add any other things you need the module to do here
    message!(env, "[{}]: binding functions...", RS_MODULE).unwrap();
    let load_doc = CString::new(format!("Load a dynamic module that defines {}.", INIT_FROM_ENV)).unwrap();
    emacs::bind_function(
        env, fn_name.to_string(),
        match n2e::function(env, 1, 1, Some(f_load_module), load_doc.as_ptr(), ptr::null_mut()) {
            Ok(x) => x,
            Err(e) => {
                message!(env, "[{}]: cannot make function '{}': {:?}", RS_MODULE, fn_name, e).unwrap();
                return 1;
            }
        },
    );

    emacs::provide(env, RS_MODULE.to_string());
    0
}

#[no_mangle]
pub extern "C" fn emacs_module_init(ert: *mut EmacsRT) -> libc::c_int {
    let env = emacs::get_environment(ert);
    load(env)
}
