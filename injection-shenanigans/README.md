# injection-shenanigans

```
     [ecosystem] <> openssl <> openssl-sys <> openssl-c

      state / test-context A - record Input / Output

[ecosystem] > [record] > [openssl] > [record] > [ecosystem]

      state / test-context A - record Input / Output

   [openssl] > [record] > [openssl-sys] > [record] > [openssl]
```

$ git clone git@github.com:sfackler/rust-openssl.git

First experiment

Make some context-isolated tests, record and replay ?

experiment if this could be used to generate Rust types




