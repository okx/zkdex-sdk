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
