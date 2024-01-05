# local build
```shell
cargo nextest run --all-features &&
cargo nextest run --examples --all-features &&
cargo test --doc --all-features &&
cargo  doc --all-features &&
cargo clippy --all-features -- --deny warnings
```

# python
```shell
cargo test --no-default-features # not this
```


# Expand Model
```shell
cargo expand --package ouch_model
```