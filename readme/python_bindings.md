
# Python lib
## setup python & maturin for development
```shell
micromamba create --name ouch_build --yes maturin ;
micromamba create --name ouch_test --yes python=3.10
```

### build & run manual
```shell
micromamba run --name ouch_build --cwd ./bindings/python maturin develop &&
micromamba run --name ouch_build --cwd ./bindings/python python examples/02_clt2svc_connect_manual_example.py
```

### build & run auto
```shell
micromamba run --name ouch_build --cwd ./bindings/python maturin develop &&
micromamba run --name ouch_build --cwd ./bindings/python python examples/03_clt2svc_connect_auto_example.py
```


## Testing the wheel on Non Latest python

```shell
(rm -f ./target/wheels/*.whl || true) &&
micromamba run --name ouch_build --cwd ./bindings/python maturin build &&
micromamba run --name ouch_test pip install --ignore-installed  ./target/wheels/*.whl &&
micromamba run --name ouch_test --cwd ./bindings/python python examples/02_clt2svc_connect_manual_example.py
```

## Testing rust lib crate

```shell
micromamba run --name ouch_build --cwd ./bindings/python cargo test 
```