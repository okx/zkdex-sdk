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
        String json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        String sigr = "0x094a47cb182c7eb24e3c34a473def9d356bb30161179e4bbaeaa48c6d18844f8";
        String sigs = "0x05534d29f2f1d3ba474f7cec4f9f545924924e5f4261577d09ed9a85df252d5d";
        String pubKeyX = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        String pubKeyY = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";
        String hash = null;
        try {
            hash = ZKDEX.hashTransfer(json);
            boolean ret = ZKDEX.verifySignature(sigr, sigs, pubKeyX, pubKeyY, hash);
            assert ret;
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    @Benchmark
    public void benchSignTransfer() {
        String priKey = "0x01e1b55a539517898350ca915cbf8b25b70d9313a5ab0ff0a3466ed7799f11fe";
        String json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        String sigStr = null;
        try {
            sigStr = ZKDEX.signTransfer(json, priKey);
            assert !sigStr.isEmpty();
        } catch (Exception e) {
            e.printStackTrace();
        }

    }

    public static void main(String[] args) throws RunnerException {
        Options opts = new OptionsBuilder().include(ZKDEXBenchmark.class.getSimpleName()).resultFormat(ResultFormatType.JSON).build();
        new Runner(opts).run();
    }
}
