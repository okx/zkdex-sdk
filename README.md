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

|      | rust                                 | javascript                                    |
| ---- | ------------------------------------ | --------------------------------------------- |
| 1    | sign: 1,634,921 ns/iter (+/- 14,055) | sign: 7,407,407 ns/iter(135 ops/sec ±0.11%)   |
|      | veirfy: 643,361 ns/iter (+/- 6,117)  | verify: 4,975,124 ns/iter(201 ops/sec ±0.05%) |
| 2    | sign: 1,628,872 ns/iter (+/- 15,966) | sign: 7,352,941ns/iter(136 ops/sec ±0.05%)    |
|      | verify: 643,267 ns/iter (+/- 4,874)  | verify: 4,975,124 ns/iter(201ops/sec ±0.04%)  |
| 3    | sign: 1,628,538 ns/iter (+/- 14,439) | sign: 7,352,941 ns/iter(136 ops/sec ±0.05%)   |
|      | verify: 642,033 ns/iter (+/- 3,454)  | verify: 5,000,000 ns/iter(200 ops/sec ±0.06%) |

