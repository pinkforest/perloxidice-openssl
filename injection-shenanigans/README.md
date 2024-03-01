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

## What we need ?

### Record `openssl` <> `openssl-sys`

We need to generate a recorder proxy crate `ffi-record-openssl-sys`

And `openssl` needs to redirect via `ffi-record-openssl-sys` to `openssl-sys`.

`ffi-record-openssl-sys` crate needs to reflect `openssl-sys` whilst recording everything between.

Experiment with `[build-dependencies]` that builds the `ffi-record-openssl-sys` from `openssl-sys` transparently.

### Record `openssl-sys` <> `openssl-c`

We already have FFI IR - we should probably reflect the associated C source so it can be annotated to inline asm.

TBD

### Generate higher level Rust types

Experiment if we create contextual tests and whether this can be used to create higher level Rust types to replace interface/s.

This could enable migration relevant C to higher level Rust potentially perhaps and split up large C codebases into smaller Rust ones.

Simulating tests would allow us to see what the input/output is in given relevant state / context - which then could reflect the logically discrete generated Rust crates + types.

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

