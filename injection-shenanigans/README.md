# injection-shenanigans

```
     [ecosystem] <> openssl <> openssl-sys <> openssl-c

      state / test-context A - record Input / Output

[ecosystem] > [record] > [openssl] > [record] > [ecosystem]

      state / test-context A - record Input / Output

   [openssl] > [record] > [openssl-sys] > [record] > [openssl]

      state / test-context A - record Input / Output

   [openssl-sys] > [record] > [openssl-c] > [record] > [openssl-sys]

```

## What can rustc tell us ?

```
--emit = [asm|llvm-bc|llvm-ir|obj|metadata|link|dep-info|mir]

$ cd rust-openssl/openssl-sys
$ cargo rustc --features bindgen -- --emit=mir
$ ls -la ../target/debug/deps/openssl-sys-*.mir

$ cargo rustc --features bindgen -- --emit=llvm-ir
$ ls -la ../target/debug/deps/openssl-sys-*.ll

$ cargo rustc --features bindgen -- --emit=llvm-bc
$ ls -la ../target/debug/deps/openssl-sys-*.bc

$ cargo rustc --features bindgen -- --emit=metadata
$ ls -la ../target/debug/deps/libopenssl_sys-*.rmeta
```

## Bootstrap

$ git clone git@github.com:sfackler/rust-openssl.git

# Methdology

## FFI Record - [openssl] <> [openssl-c]

1. point openssl ffi from openssl-sys to ffi-record-openssl-sys


# Experiments

##  First experiment

Make some context-isolated tests, record and replay ?

experiment if this could be used to generate Rust types





```

