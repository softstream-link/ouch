# Local build
```shell
cargo nextest run --all-features ; \
cargo nextest run --examples ; \
cargo test --doc ; \
cargo doc ; \
cargo clippy --all-features -- --deny warnings
```

# Python lib
## setup python & maturin
```shell
micromamba create --name ouch --yes maturin
micromamba deactivate
```

## build & run
```shell
micromamba activate ouch ; \
pushd connect/nonblocking_python ; \
maturin develop ; \
python examples/clt2svc_connect_example.py ; \
popd ; \
micromamba deactivate
```

# Expand Model
```shell
cargo expand --package ouch_model
```