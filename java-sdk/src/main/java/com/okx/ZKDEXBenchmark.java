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
    public void benchVerifyTransfer() throws Exception {
        String json = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        String sigr = "0x1c929aba1dd2f9cacf5c857e014b2ea1bbd98e5758821a20293b12c869e51732";
        String sigs = "0x03d739463c57a40e49b8e52f54c18acce5f205ee9ffcee2b96ac83bc3fbcf476";
        String pubKeyX = "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a";
        String pubKeyY = "0x0a3b966094be6c8981a22359df81f7fcdd50ac725401e3fc5872c780d158fb18";
        String hash = null;
        try {
            hash = ZKDEX.hashTransfer(json);
            boolean ret = ZKDEX.verifySignature(sigr, sigs, pubKeyX, pubKeyY, hash);
            assert ret;
        } catch (Exception e) {
            e.printStackTrace();
            throw e;
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
