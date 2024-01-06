# local build
```shell
cargo nextest run --all-features &&
cargo nextest run --examples --all-features &&
cargo test --doc --all-features &&
cargo doc --all-features --no-deps &&
cargo clippy --all-features -- --deny warnings
```

# local build
```shell
micromamba create --file ./bindings/python/micromamba/ouch_build_env.yml --yes &&
micromamba activate ouch_build_env &&
cargo nextest run --all-features &&
cargo nextest run --examples --all-features &&
cargo test --doc --all-features &&
cargo doc --all-features --no-deps &&
cargo clippy --all-features -- --deny warnings
(
    pushd ./bindings/python && 
    maturin develop && 
    pytest ./tests ; 
    popd 
) &&
micromamba deactivate
```
## Testing the wheel on Non Latest python

```shell
(rm -f ./target/wheels/*.whl || true) &&
micromamba run --name ouch_build --cwd ./bindings/python maturin build &&
micromamba run --name ouch_test pip install --ignore-installed  ./target/wheels/*.whl &&
micromamba run --name ouch_test --cwd ./bindings/python python examples/02_clt2svc_connect_manual_example.py
```


# Expand Model
```shell
cargo expand --package ouch_model
```