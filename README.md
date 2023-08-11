# zkWasm-sdk

## how to use
1. clone from git
```shell
git clone https://github.com/okx/zkwasm-sdk.git
```
2. compile rust to npm package
```
cd zkwasm-sdk
git checkout new_hash
cd rust-sdk && ./build.sh
```
3. run js example

```shell
cd zkwasm-sdk/js-example
npm i && npm run test
```
## benchmark

machine: **16c 64g**

> The process of verifying a signature involves a hash calculation.

|      | rust                | javascript          | java                |
| ---- | ------------------- | ------------------- |---------------------|
| 1    | sign: 1.63 ms /op   | sign: 7.40 ms /op   | sign: 1.63 ms /op   |
|      | veirfy: 1.04 ms /op | verify: 4.97 ms /op | verify: 1.04 ms /op |
| 2    | sign: 1.62 ms /op   | sign: 7.35 ms /op   | sign: 1.63 ms /op   |
|      | verify: 1.04 ms /op | verify: 4.97 ms /op | verify: 1.04 ms/op  |
| 3    | sign: 1.62 ms /op   | sign: 7.35 ms /op   | sign: 1.63 ms /op   |
|      | verify: 1.03ms /op  | verify: 5.00 ms /op | verify: 1.04 ms /op |

