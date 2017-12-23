#!/usr/bin/env bash

# Load the dynamic module into

here=`cd $(dirname $BASH_SOURCE); pwd`
root=`cd $here/..; pwd`
RS_MODULE=$(find $root -iname '*emacs_rs_module*.dylib' | head -n 1)
MODULE=$root/target/debug/libmagit_libgit2.dylib

read -r -d '' expr <<EOF
(progn
  (unless (featurep 'rs-module)
    (module-load "$RS_MODULE"))
  (rs-module/load "$MODULE"))
EOF

emacsclient -e "$expr"

echo '~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~'
