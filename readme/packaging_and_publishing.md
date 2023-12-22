# Local build
```shell
cargo nextest run --all-features ; \
cargo nextest run --examples ; \
cargo test --doc ; \
cargo doc ; \
cargo clippy --all-features -- --deny warnings
```


# Expand Model
```shell
cargo expand --package ouch_model
```