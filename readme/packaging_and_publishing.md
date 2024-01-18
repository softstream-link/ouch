# Local build & test rust only
* `ouch_bindings_python` **REQUIRES** system python in order to compile  
```shell
cargo nextest run --all-features &&
cargo nextest run --examples --all-features &&
cargo test --doc --all-features &&
cargo doc --all-features --no-deps &&
cargo clippy --all-features -- --deny warnings
```

# Local build & test rust & python extension
* `ouch_bindings_python` will use `micromamba` env which has `python, maturin, pytest`
```shell
micromamba create --name ouch_build_env --yes python maturin pytest &&
micromamba run --name ouch_build_env cargo nextest run --all-features &&
micromamba run --name ouch_build_env cargo nextest run --examples --all-features && 
micromamba run --name ouch_build_env cargo test --doc --all-features &&
micromamba run --name ouch_build_env cargo clippy --all-features -- --deny warnings &&
micromamba run --name ouch_build_env cargo doc --all-features &&
micromamba run --name ouch_build_env --cwd ./bindings/python maturin develop &&
micromamba run --name ouch_build_env --cwd ./bindings/python pytest
```

# Regenerate `ouch_connect.pyi` file
```shell    
micromamba run --name ouch_build_env --cwd ./bindings/python/ouch_connect pip install cogapp
micromamba run --name ouch_build_env --cwd ./bindings/python/ouch_connect cog -r ouch_connect.pyi
```

# Testing python extension
* test with minimum python version `3.10`
* NOTE: must have `ouch_build_env` already created from prior step
```shell
micromamba create --name ouch_test_env --yes python=3.10 &&
(rm -f ./target/wheels/*.whl || true) &&
micromamba run --name ouch_build_env --cwd ./bindings/python maturin build &&
micromamba run --name ouch_test_env  pip install --ignore-installed ./target/wheels/*.whl &&
for py in `ls ./bindings/python/tests/*.py` ; do echo "************* $py **************"; micromamba run --name ouch_test_env  python $py ; done
```

# Testing pypi wheel
```shell
micromamba create --name ouch_pypi_env --yes python=3.10
micromamba run --name ouch_pypi_env pip install ouch-connect
for py in `ls ./bindings/python/tests/*.py` ; do echo "************* $py **************"; micromamba run --name ouch_pypi_env  python $py ; done
```

# Expand Model
```shell
cargo expand --package ouch_model
```

