package com.okx;

import org.openjdk.jmh.annotations.*;
import org.openjdk.jmh.results.format.ResultFormatType;
import org.openjdk.jmh.runner.Runner;
import org.openjdk.jmh.runner.RunnerException;
import org.openjdk.jmh.runner.options.Options;
import org.openjdk.jmh.runner.options.OptionsBuilder;

import java.util.concurrent.TimeUnit;

@BenchmarkMode(Mode.AverageTime)
@OutputTimeUnit(TimeUnit.NANOSECONDS)
@State(Scope.Thread)
@Warmup(iterations = 3, time = 1, timeUnit = TimeUnit.SECONDS)
@Measurement(iterations = 5, time = 1, timeUnit = TimeUnit.SECONDS)
@Fork(1)
@Threads(2)
public class ZKDEXBenchmark {

    @Benchmark
    public void benchVerifyTransfer() {
        String sigr = "353b5e0902f1918f2a5ed18d190c90d4c5bc0267566030283ecb996d2e4443a6";
        String sigs = "c80432d841049c2e71fcb590ff6ebcde58ae7cc1f064460bb4de474f93050502";
        String pubKey = "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9";
        String json = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}";
        String hash = ZKDEX.hashTransfer(json);

        boolean ret = ZKDEX.verifySignature(sigr, sigs, pubKey, hash);
        assert ret;
    }

    @Benchmark
    public void benchSignTransfer() {
        String priKey = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
        String json = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}";
        String sigStr = ZKDEX.signTransfer(json, priKey);
        assert !sigStr.isEmpty();
    }

    public static void main(String[] args) throws RunnerException {
        Options opts = new OptionsBuilder().include(ZKDEXBenchmark.class.getSimpleName()).resultFormat(ResultFormatType.JSON).build();
        new Runner(opts).run();
    }
}
