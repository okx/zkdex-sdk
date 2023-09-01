/* tslint:disable */
/* eslint-disable */
/**
* This method initializes params for current thread, otherwise they will be initialized when signing
* first message.
*/
export function zkdex_init(): void;
/**
* @param {Uint8Array} pubkey
* @returns {Uint8Array}
*/
export function pubKeyHash(pubkey: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} private_key
* @returns {Uint8Array}
*/
export function private_key_to_pubkey_hash(private_key: Uint8Array): Uint8Array;
/**
* sign a transfer transaction
* @param {string} json
* @param {string} private_key
* @returns {string}
*/
export function sign_transfer(json: string, private_key: string): string;
/**
* hash a transfer transaction
* @param {string} json
* @returns {string}
*/
export function hash_transfer(json: string): string;
/**
* sign a withdraw transaction
* @param {string} json
* @param {string} private_key
* @returns {string}
*/
export function sign_withdraw(json: string, private_key: string): string;
/**
* hash a withdraw transaction
* @param {string} json
* @returns {string}
*/
export function hash_withdraw(json: string): string;
/**
* sign a limit order transaction
* @param {string} json
* @param {string} private_key
* @returns {string}
*/
export function sign_limit_order(json: string, private_key: string): string;
/**
* sign a limit order transaction
* @param {string} json
* @returns {string}
*/
export function hash_limit_order(json: string): string;
/**
* sign a liquidate transaction
* @param {string} json
* @param {string} private_key
* @returns {string}
*/
export function sign_liquidate(json: string, private_key: string): string;
/**
* hash a liquidate transaction
* @param {string} json
* @returns {string}
*/
export function hash_liquidate(json: string): string;
/**
* sign a signed oracle price transaction
* @param {string} json
* @param {string} private_key
* @returns {string}
*/
export function sign_signed_oracle_price(json: string, private_key: string): string;
/**
* hash a signed oracle price transaction
* @param {string} json
* @returns {string}
*/
export function hash_signed_oracle_price(json: string): string;
/**
* verify a signature
* @param {string} sig_r
* @param {string} sig_s
* @param {string} pub_key_x
* @param {string} pub_key_y
* @param {string} msg
* @returns {boolean}
*/
export function verify_signature(sig_r: string, sig_s: string, pub_key_x: string, pub_key_y: string, msg: string): boolean;
/**
* l1 sign
* sign a msg on l1 when signing a eth address
* @param {string} msg
* @param {string} private_key
* @returns {string}
*/
export function l1_sign(msg: string, private_key: string): string;
/**
* check the (x,y) is on curve
* @param {string} pub_key_x
* @param {string} pub_key_y
* @returns {boolean}
*/
export function is_on_curve(pub_key_x: string, pub_key_y: string): boolean;
/**
* sign a msg on l2
* @param {string} pri_key
* @param {string} msg
* @returns {string}
*/
export function sign(pri_key: string, msg: string): string;
/**
* derive a private key from a random seed, the seed could be anything
* @param {string} seed
* @returns {string}
*/
export function private_key_from_seed(seed: string): string;
/**
* derive a public with xy from private key
* @param {string} pri_key
* @returns {string}
*/
export function private_key_to_pubkey_xy(pri_key: string): string;
/**
* convert public key to xy
* @param {string} pub_key
* @returns {string}
*/
export function public_key_to_xy(pub_key: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly Java_com_okx_ZKDEX_verifySignature: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly Java_com_okx_ZKDEX_signWithdraw: (a: number, b: number, c: number, d: number) => number;
  readonly Java_com_okx_ZKDEX_signTransfer: (a: number, b: number, c: number, d: number) => number;
  readonly Java_com_okx_ZKDEX_signLimitOrder: (a: number, b: number, c: number, d: number) => number;
  readonly Java_com_okx_ZKDEX_signLiquidate: (a: number, b: number, c: number, d: number) => number;
  readonly Java_com_okx_ZKDEX_signSignedOraclePrice: (a: number, b: number, c: number, d: number) => number;
  readonly Java_com_okx_ZKDEX_hashWithdraw: (a: number, b: number, c: number) => number;
  readonly Java_com_okx_ZKDEX_hashTransfer: (a: number, b: number, c: number, d: number) => number;
  readonly Java_com_okx_ZKDEX_hashLimitOrder: (a: number, b: number, c: number) => number;
  readonly Java_com_okx_ZKDEX_hashLiquidate: (a: number, b: number, c: number) => number;
  readonly Java_com_okx_ZKDEX_hashSignedOraclePrice: (a: number, b: number, c: number) => number;
  readonly Java_com_okx_ZKDEX_sign: (a: number, b: number, c: number, d: number) => number;
  readonly Java_com_okx_ZKDEX_privateKeyFromSeed: (a: number, b: number, c: number) => number;
  readonly Java_com_okx_ZKDEX_isOnCurve: (a: number, b: number, c: number, d: number) => number;
  readonly Java_com_okx_ZKDEX_privateKeyToPublicKeyXY: (a: number, b: number, c: number) => number;
  readonly Java_com_okx_ZKDEX_publicKeyToXY: (a: number, b: number, c: number) => number;
  readonly zkdex_init: () => void;
  readonly pubKeyHash: (a: number, b: number, c: number) => void;
  readonly private_key_to_pubkey_hash: (a: number, b: number, c: number) => void;
  readonly sign_transfer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly hash_transfer: (a: number, b: number, c: number) => void;
  readonly sign_withdraw: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly hash_withdraw: (a: number, b: number, c: number) => void;
  readonly sign_limit_order: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly hash_limit_order: (a: number, b: number, c: number) => void;
  readonly sign_liquidate: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly hash_liquidate: (a: number, b: number, c: number) => void;
  readonly sign_signed_oracle_price: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly hash_signed_oracle_price: (a: number, b: number, c: number) => void;
  readonly verify_signature: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly l1_sign: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly is_on_curve: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly sign: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly private_key_from_seed: (a: number, b: number, c: number) => void;
  readonly private_key_to_pubkey_xy: (a: number, b: number, c: number) => void;
  readonly public_key_to_xy: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
