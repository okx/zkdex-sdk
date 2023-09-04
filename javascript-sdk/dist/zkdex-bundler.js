import * as wasm from "./zkdex-bundler_bg.wasm";
import { __wbg_set_wasm } from "./zkdex-bundler_bg.js";
__wbg_set_wasm(wasm);
export * from "./zkdex-bundler_bg.js";

wasm.__wbindgen_start();
