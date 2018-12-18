UEFI libraries and examples for Rust.

## Build

* Use [xargo] or [cargo-xbuild] to build.
* Use nightly compiler.
* Target is set to `x86_64-unknown-uefi`.

```
cargo install cargo-xbuild
cargo xbuild --example minimal
```

[xargo]: https://github.com/japaric/xargo
[cargo-xbuild]: https://github.com/rust-osdev/cargo-xbuild
