# zkWasm-sdk

## how to use
1. clone from okx
```shell
git clone https://github.com/okx/zkwasm-sdk.git
```
2. compile rust to npm package
```
cd zkwasm-sdk
git checkout new_hash
cd zksync-crypto && ./build.sh
```
3. run js example
```shell
cd ../js-example
npm i && npm run dev
```