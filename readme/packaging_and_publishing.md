# local build Rust only, does need system python in order to compile ouch_bindings_python 
```shell
cargo nextest run --all-features &&
cargo nextest run --examples --all-features &&
cargo test --doc --all-features &&
cargo doc --all-features --no-deps &&
cargo clippy --all-features -- --deny warnings
```

# local build on micromamba python with python tests
```shell
micromamba create --name ouch_build_env --yes maturin pytest &&
micromamba run --name ouch_build_env cargo nextest run --all-features &&
micromamba run --name ouch_build_env cargo nextest run --examples --all-features && 
micromamba run --name ouch_build_env cargo test --doc --all-features &&
micromamba run --name ouch_build_env cargo clippy --all-features -- --deny warnings
micromamba run --name ouch_build_env --cwd ./bindings/python maturin develop
micromamba run --name ouch_build_env --cwd ./bindings/python pytest ./tests
```
# Testing the wheel on Non Latest python
## create ouch_build_env
```shell
micromamba create --name ouch_build_env --yes maturin pytest
```
## create ouch_test_env & run example
```shell
micromamba create --name ouch_test_env --yes python=3.10 &&
(rm -f ./target/wheels/*.whl || true) &&
micromamba run --name ouch_build_env --cwd ./bindings/python maturin build --out ./../../target/wheels &&
micromamba run --name ouch_test_env  pip install --ignore-installed ./target/wheels/*.whl &&
micromamba run --name ouch_test_env  python ./bindings/python/tests/02_clt2svc_connect_manual_test.py
for py in `ls ./bindings/python/tests/*.py` ; do echo "************* $py **************"; micromamba run --name ouch_test_env  python $py ; done
```


# Expand Model
```shell
cargo expand --package ouch_model
```