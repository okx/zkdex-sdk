/* tslint:disable */
/* eslint-disable */
/**
* sign a transfer transaction
* @param {string} json of transfer transaction
* @param {string} private key hex with 0x prefix
* @returns {string} signature of transfer transaction
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
/**
* This method initializes params for current thread, otherwise they will be initialized when signing
* first message.
*/
export function zkdex_init(): void;
