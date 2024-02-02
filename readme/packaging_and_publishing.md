# Local build & test rust only
* `ouch_bindings_python` **REQUIRES** system python in order to compile  
```shell
if [ -d ./ouch_connect ] ; then PREFIX="./../.." ; else PREFIX="." fi
pushd ${PREFIX}
cargo nextest run --all-features &&
cargo nextest run --examples --all-features &&
cargo test --doc --all-features &&
cargo doc --all-features --no-deps &&
cargo clippy --all-features -- --deny warnings
```

# Local build & test rust & python extension
* `ouch_bindings_python` will use `micromamba` env which has `python, maturin, pytest`
```shell
if [ -d ./ouch_connect ] ; then CWD="./../.." ; else CWD=".";  fi
cd ${CWD}
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
if [ -d ./ouch_connect ] ; then CWD="./../.." ; else CWD=".";  fi
cd ${CWD}
micromamba run --name ouch_build_env pip install cogapp
micromamba run --name ouch_build_env cog -r ./bindings/python/ouch_connect/ouch_connect.pyi
```

# Testing python extension
* test with minimum python version `3.11`
* NOTE: must have `ouch_build_env` already created from prior step
```shell
if [ -d ./ouch_connect ] ; then PREFIX="./../.." ; else PREFIX="." fi
micromamba create --name ouch_test_env --yes python=3.11 pytest &&
(rm -f ./target/wheels/*.whl || true) &&
micromamba run --name ouch_build_env --cwd ${PREFIX}/bindings/python maturin build &&
micromamba run --name ouch_test_env  --cwd ${PREFIX} pip install --ignore-installed ./target/wheels/*.whl &&
micromamba run --name ouch_test_env  --cwd ${PREFIX}/bindings/python pytest
```
<!-- for py in `ls ./bindings/python/tests/*.py` ; do echo "************* $py **************"; micromamba run --name ouch_test_env  python $py ; done -->

# Testing pypi wheel
```shell
if [ -d ./ouch_connect ] ; then PREFIX="./../.." ; else PREFIX="." fi
micromamba create --name ouch_pypi_env --yes python=3.11
micromamba run --name ouch_pypi_env  --cwd ${PREFIX}/bindings/python pip install ouch-connect
micromamba run --name ouch_pypi_env  --cwd ${PREFIX}/bindings/python pytest

```

# Expand Model
```shell
cargo expand --package ouch_model
```

# Ubuntu Pod 
## Build image
* goto `links` project and run a section `docker build ...` from `<links>/readme/dev-how-to/ubuntu-pod/readme.md`

## To run
```shell
# the cap-add are required for tshark to see eth0 and other network interfaces
docker run \
    --rm --interactive --tty \
    --user "$(id -u)":"$(id -g)" \
    --volume "$(pwd)/..":/home/$(whoami)/dev \
    --workdir /home/$(whoami)/dev \
    --name links_on_ubuntu_pod \
    --cap-add=NET_RAW --cap-add=NET_ADMIN -it \
    links_on_ubuntu_image
```

## To run tests
```shell
docker exec \
    --interactive --tty \
    links_on_ubuntu_pod \
    bash -c " \
    rustup default stable ; \
    pushd ouch ; \
    cargo nextest run --all-features ; \
    cargo nextest run --examples --all-features ; \
    cargo test --doc --all-features; \
    cargo doc --all-features; \
    "
```