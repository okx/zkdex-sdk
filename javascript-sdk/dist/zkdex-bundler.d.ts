/* tslint:disable */
/* eslint-disable */
/**
* This method initializes params for current thread, otherwise they will be initialized when signing
* first message.
*/
export function zksync_crypto_init(): void;
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
* @param {string} json
* @param {string} private_key
* @returns {string}
*/
export function sign_transfer(json: string, private_key: string): string;
/**
* @param {string} json
* @returns {string}
*/
export function hash_transfer(json: string): string;
/**
* @param {string} json
* @param {string} asset_id_collateral
* @param {string} private_key
* @returns {string}
*/
export function sign_withdraw(json: string, asset_id_collateral: string, private_key: string): string;
/**
* @param {string} json
* @param {string} asset_id_collateral
* @returns {string}
*/
export function hash_withdraw(json: string, asset_id_collateral: string): string;
/**
* @param {string} json
* @param {string} private_key
* @returns {string}
*/
export function sign_limit_order(json: string, private_key: string): string;
/**
* @param {string} json
* @returns {string}
*/
export function hash_limit_order(json: string): string;
/**
* @param {string} json
* @param {string} private_key
* @returns {string}
*/
export function sign_liquidate(json: string, private_key: string): string;
/**
* @param {string} json
* @returns {string}
*/
export function hash_liquidate(json: string): string;
/**
* @param {string} json
* @param {string} private_key
* @returns {string}
*/
export function sign_signed_oracle_price(json: string, private_key: string): string;
/**
* @param {string} json
* @returns {string}
*/
export function hash_signed_oracle_price(json: string): string;
/**
* @param {string} sig_r
* @param {string} sig_s
* @param {string} pub_key_x
* @param {string} pub_key_y
* @param {string} msg
* @returns {boolean}
*/
export function verify_signature(sig_r: string, sig_s: string, pub_key_x: string, pub_key_y: string, msg: string): boolean;
/**
* @param {string} msg
* @param {string} private_key
* @returns {string}
*/
export function l1_sign(msg: string, private_key: string): string;
/**
* @param {string} pub_key_x
* @param {string} pub_key_y
* @returns {boolean}
*/
export function is_on_curve(pub_key_x: string, pub_key_y: string): boolean;
/**
* @param {string} pri_key
* @param {string} msg
* @returns {string}
*/
export function sign(pri_key: string, msg: string): string;
/**
* @param {string} seed
* @returns {string}
*/
export function private_key_from_seed(seed: string): string;
/**
* @param {string} pri_key
* @returns {string}
*/
export function private_key_to_pubkey_xy(pri_key: string): string;
