
# Python lib
## setup python & maturin for development
```shell
micromamba create --name ouch_build --yes maturin ; \
micromamba create --name ouch_test --yes python=3.10
```

### build & run manual
```shell
micromamba run --name ouch_build --cwd ./connect/nonblocking_python maturin develop ; \
micromamba run --name ouch_build --cwd ./connect/nonblocking_python python examples/clt2svc_connect_manual_example.py
```

### build & run auto
```shell
micromamba run --name ouch_build --cwd ./connect/nonblocking_python maturin develop ; \
micromamba run --name ouch_build --cwd ./connect/nonblocking_python python examples/clt2svc_connect_auto_example.py
```


## Testing the wheel

```shell
micromamba run --name ouch_build --cwd ./connect/nonblocking_python maturin build ; \
micromamba run --name ouch_test pip install --ignore-installed  ./target/wheels/ouch_connect_nonblocking_python-5.0.0-cp310-abi3-macosx_11_0_arm64.whl
micromamba run --name ouch_test --cwd ./connect/nonblocking_python python examples/clt2svc_connect_manual_example.py
```