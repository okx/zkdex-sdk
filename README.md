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
| 1    | sign: 1,641,654 ns/iter (+/- 16,401) | sign: 7,407,407 ns/iter(135 ops/sec ±0.11%)   |
|      | veirfy: 635,971 ns/iter (+/- 5,330)  | verify: 4,975,124 ns/iter(201 ops/sec ±0.05%) |
| 2    | sign: 1,639,635 ns/iter (+/- 19,013) | sign: 7,352,941ns/iter(136 ops/sec ±0.05%)    |
|      | verify: 635,582 ns/iter (+/- 15,485) | verify: 4,975,124 ns/iter(201ops/sec ±0.04%)  |
| 3    | sign: 1,639,752 ns/iter (+/- 14,824) | sign: 7,352,941 ns/iter(136 ops/sec ±0.05%)   |
|      | verify: 633,977 ns/iter (+/- 2,376)  | verify: 5,000,000 ns/iter(200 ops/sec ±0.06%) |

