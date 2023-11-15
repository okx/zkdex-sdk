# ZKDEX-Python-SDK
This is a python sdk for zkdex

## How to use
1. install the package
```shell
pip install zkdex-python-sdk
```

## How to develop
1. create a virtual environment
```shell
python3 -m venv .env
# activate 
source .env/bin/activate
# if you use windows, you should activate it like this:
# .\.env\Scripts\Activate.ps1
pip install maturin
```
2. build the project
```shell
maturin develop
```
3. run the test
```shell
python3 tests/test.py
```