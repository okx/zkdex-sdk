bench: java_bench


java_bench: check_env
	cd rust-sdk && cargo build --release
	cd java-sdk && mvn clean verify
	java -Djava.library.path=./rust-sdk/target/release -jar ./java-sdk/target/benchmarks.jar

check_env:
	bash scripts/check_env.sh