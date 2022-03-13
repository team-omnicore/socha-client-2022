# Build

```shell
rustup show
rustup target add x86_64-unknown-linux-musl
cargo build --target=x86_64-unknown-linux-musl --release
```

Intel® Xeon® Prozessor E5-2620 v4
CPU Server Software Challenge
```shell
RUSTFLAGS="-C opt-level=3 -C target-cpu=broadwell -C overflow-checks=no -C lto=fat -C embed-bitcode=y" cargo build --target=x86_64-unknown-linux-musl --release
```