# Emacs Rust Module

This is an Emacs dynamic module written in Rust that aims to streamline development of other Emacs dynamic modules.

## Installation
Build:

``` shell
cargo build
```

Load the module in Emacs:

```elisp
(module-load "/path/to/emacs-rs-module/target/debug/libemacs_rs_module.dylib")
```

## Live reloading another module
To be reloadable, the module must export an entry point named `emacs_rs_module_init`. See [example-rs-module](example-rs-module).

Run this in Emacs after each `cargo build` to reload the module:

```elisp
(rs-module/load "/path/to/my-module/target/debug/libmy_module.dylib")
```
