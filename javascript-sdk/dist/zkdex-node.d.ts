/* tslint:disable */
/* eslint-disable */
/**
* This method initializes params for current thread, otherwise they will be initialized when signing
* first message.
*/
export function zkdex_init(): void;
/**
* sign_transfer, sign a transfer transaction.
* @param {string} json  json of transfer transaction.
* @param {string} private_key private key hex with 0x prefix.
* @returns {string} json signature of transfer transaction.
*/
export function sign_transfer(json: string, private_key: string): string;
/**
* hash_transfer, hash a transfer transaction.
* @param {string} json  json of transfer transaction.
* @returns {string} string hash of transfer transaction with 0x prefix.
*/
export function hash_transfer(json: string): string;
/**
* sign_withdraw, sign a withdraw transaction.
* @param {string} json  json of withdraw transaction.
* @param {string} private_key private key hex with 0x prefix.
* @returns {string} json signature of withdraw transaction.
*/
export function sign_withdraw(json: string, private_key: string): string;
/**
* hash_withdraw, hash a withdraw transaction.
* @param {string} json  json of withdraw transaction.
* @returns {string} string hash of withdraw transaction with 0x prefix.
*/
export function hash_withdraw(json: string): string;
/**
* sign_limit_order, sign a limit order transaction.
* @param {string} json  json of limit order transaction.
* @param {string} private_key private key hex with 0x prefix.
* @returns {string} json signature of limit order transaction.
*/
export function sign_limit_order(json: string, private_key: string): string;
/**
* hash_limit_order, sign a limit order transaction.
* @param {string} json  json of limit order transaction.
* @returns {string} string hash of limit order transaction with 0x prefix.
*/
export function hash_limit_order(json: string): string;
/**
* sign_liquidate, sign a liquidate transaction.
* @param {string} json  json of liquidate transaction.
* @param {string} private_key private key hex with 0x prefix.
* @returns {string} json signature of liquidate transaction.
*/
export function sign_liquidate(json: string, private_key: string): string;
/**
* hash_liquidate, hash a liquidate transaction.
* @param {string} json  json of liquidate transaction.
* @returns {string} string hash of liquidate transaction with 0x prefix.
*/
export function hash_liquidate(json: string): string;
/**
* sign_signed_oracle_price, sign a signed oracle price transaction.
* @param {string} json  json of liquidate transaction.
* @param {string} private_key private key hex with 0x prefix.
* @returns {string} json signature of liquidate transaction.
*/
export function sign_signed_oracle_price(json: string, private_key: string): string;
/**
* hash_signed_oracle_price, hash a signed oracle price transaction.
* @param {string} json  json of signed oracle transaction.
* @returns {string} string hash of signed oracle transaction with 0x prefix.
*/
export function hash_signed_oracle_price(json: string): string;
/**
* verify_signature, verify a signature.
* @param {string} sig_r  r of signature.
* @param {string} sig_s  s of signature.
* @param {string} pub_key_x  x of public key.
* @param {string} pub_key_y  y of public key.
* @param {string} msg  msg hex with 0x prefix.
* @returns {bool} whether the signature is valid.
*/
export function verify_signature(sig_r: string, sig_s: string, pub_key_x: string, pub_key_y: string, msg: string): boolean;
/**
* l1 sign, sign a msg on l1 when signing a eth address.
* @param {string} msg  msg coding in hex with 0x prefix.
* @param {string} private_key private key hex with 0x prefix.
* @param {string} string of signature.
*/
export function l1_sign(msg: string, private_key: string): string;
/**
* is_on_curve, check the (x,y) is on curve.
* @param {string} pub_key_x  x of public key with 0x prefix.
* @param {string} pub_key_y  y of public key with 0x prefix.
* @returns {bool} whether the (x,y) is on curve.
*/
export function is_on_curve(pub_key_x: string, pub_key_y: string): boolean;
/**
* sign, sign a msg on l2, is a generic signature methods.
* @param {string} msg  msg coding in hex with 0x prefix.
* @param {string} private_key private key hex with 0x prefix.
* @returns {string} json string of the signature.
*/
export function sign(private_key: string, msg: string): string;
/**
* private_key_from_seed, derive a private key from a random seed, the seed could be anything.
* @param {string} seed  anything string.
* @returns {string} string of private coding in hex with 0x prefix.
*/
export function private_key_from_seed(seed: string): string;
/**
* private_key_to_pubkey_xy, derive a public with xy from private key.
* @param {string} private_key private key hex with 0x prefix.
* @returns {string} json string of public key xy.
*/
export function private_key_to_pubkey_xy(pri_key: string): string;
/**
* public_key_to_xy, convert public key to xy.
* @param {string} pub_key public key hex with 0x prefix.
* @returns {string} json string of public key xy.
*/
export function public_key_to_xy(pub_key: string): string;
/**
*/
export function zkmain(): void;
