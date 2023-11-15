bench: java_bench rust_bench java_script_bench

all: check_env java_sdk java_script_sdk

java_bench: check_env java_sdk
	java -Djava.library.path=./rust-sdk/target/release -jar ./java-sdk/target/benchmarks.jar

rust_bench: check_env
	cd rust-sdk && cargo bench

check_env:
	bash scripts/check_env.sh

java_script_sdk:
	cd rust-sdk && npm run build
java_sdk:
	cd rust-sdk && cargo build --release
	cd java-sdk && mvn clean verify
java_script_bench: java_script_sdk
	cd js-example && npm i && npm run bench

mac_lib:
	cd rust-sdk && cargo build --features java --release
	cd rust-sdk && cargo build --features java --target=x86_64-apple-darwin --release
	cp -f rust-sdk/target/release/libzkdex_sdk.dylib java-sdk/src/main/resources/com/okx/arm_libzkdex_sdk.dylib
	cp -f rust-sdk/target/x86_64-apple-darwin/release/libzkdex_sdk.dylib java-sdk/src/main/resources/com/okx/x86_64_libzkdex_sdk.dylib
linux_lib:
	cd rust-sdk && cross build --features java --target x86_64-unknown-linux-gnu --release
	cp -f rust-sdk/target/x86_64-unknown-linux-gnu/release/libzkdex_sdk.so java-sdk/src/main/resources/com/okx/libzkdex_sdk.so
win_lib:
	cd rust-sdk && cargo build --features java --release
	# windows 下复制文件
	cp .\rust-sdk\target\release\zkdex_sdk.dll .\java-sdk\src\main\resources\com\okx\zkdex_sdk.dll