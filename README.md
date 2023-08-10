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

|      | rust                                   | javascript                                    |
| ---- | -------------------------------------- | --------------------------------------------- |
| 1    | sign: 1,630,130 ns/iter (+/- 15,594)   | sign: 7,407,407 ns/iter(135 ops/sec ±0.11%)   |
|      | veirfy: 1,045,030 ns/iter (+/- 7,770)  | verify: 4,975,124 ns/iter(201 ops/sec ±0.05%) |
| 2    | sign: 1,623,271 ns/iter (+/- 15,750)   | sign: 7,352,941ns/iter(136 ops/sec ±0.05%)    |
|      | verify: 1,045,514 ns/iter (+/- 35,496) | verify: 4,975,124 ns/iter(201ops/sec ±0.04%)  |
| 3    | sign: 1,626,561 ns/iter (+/- 14,677)   | sign: 7,352,941 ns/iter(136 ops/sec ±0.05%)   |
|      | verify: 1,039,647 ns/iter (+/- 26,029) | verify: 5,000,000 ns/iter(200 ops/sec ±0.06%) |

