# zkdex-sdk

> zkdex sdk contains rust-sdk、java-sdk、javascript-sdk

Get zkdex-sdk from github

```shell
git clone https://github.com/okx/zkdex-sdk.git
```

## Getting start with javascript-sdk

1. compile rust to npm package

```
cd zkdex-sdk
make all
```
2. run js example

```shell
cd zkdex-sdk/js-example
npm i && npm run test
```
3. Please refer to the javascript-sdk‘s REAMDE for detailed api.

## Getting start with java-sdk

1. Compile the dynamic libraries, it's in . /rust-sdk/target/release directory. 

```
cd zkdex-sdk
make all
```

2. Create a new maven project and then import zkdex-java-sdk-1.0-SNAPSHOT.jar in ./java-sdk/target directory.
3. Write a demo code

```java
import com.okx.ZKDEX;

public static void main(String[] args) throws RunnerException {
       String priKey = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        String json = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}";
        String sigStr = null;
        try {
            sigStr = ZKDEX.signTransfer(json, priKey);
            assert !sigStr.isEmpty();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
```

4. before runing the demo, use JVM params   -Djava.library.path  to set the location of dynamic libraries. like this:

```
java -Djava.library.path=./rust-sdk/target/release
```

5. Please refer to the java-sdk‘s REAMDE for detailed api.

## Benchmark

machine: **16c 64g**

> The process of verifying a signature involves a hash calculation.

|      | rust                | javascript          | java                |
| ---- | ------------------- | ------------------- | ------------------- |
| 1    | sign: 1.63 ms /op   | sign: 7.40 ms /op   | sign: 1.63 ms /op   |
|      | veirfy: 1.04 ms /op | verify: 4.97 ms /op | verify: 1.04 ms /op |
| 2    | sign: 1.62 ms /op   | sign: 7.35 ms /op   | sign: 1.63 ms /op   |
|      | verify: 1.04 ms /op | verify: 4.97 ms /op | verify: 1.04 ms/op  |
| 3    | sign: 1.62 ms /op   | sign: 7.35 ms /op   | sign: 1.63 ms /op   |
|      | verify: 1.03ms /op  | verify: 5.00 ms /op | verify: 1.04 ms /op |
