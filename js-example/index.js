import * as wasm from 'zksync-crypto';
import { ethers as utils } from 'ethers';
function hexStringToUint8Array(hexString){
    if (hexString.length % 2 !== 0){
        throw "Invalid hexString";
    }/*from  w w w.  j  av a 2s  . c  o  m*/
    var arrayBuffer = new Uint8Array(hexString.length / 2);

    for (var i = 0; i < hexString.length; i += 2) {
        var byteValue = parseInt(hexString.substr(i, 2), 16);
        if (isNaN(byteValue)){
            throw "Invalid hexString";
        }
        arrayBuffer[i/2] = byteValue;
    }

    return arrayBuffer;
}


const yz_priv = "05510911e24cade90e206aabb9f7a03ecdea26be4a63c231fabff27ace91471e";
const yz_priv_bytes = hexStringToUint8Array(yz_priv);

const v=wasm.privateKeyFromSeed(yz_priv_bytes);
console.log('asd ',v);

const signaturePacked =  wasm.sign_musig(yz_priv_bytes, hexStringToUint8Array("abcd"));
const pubKey = utils.hexlify(signaturePacked.slice(0, 32)).substr(2);
const signature = utils.hexlify(signaturePacked.slice(32)).substr(2);
console.log('signature.pubKey', pubKey);
console.log('signature.signature', signature);


const signaturePackedWithoutHash =  wasm.sign_musig_without_hash_msg(yz_priv_bytes, hexStringToUint8Array("abcd"));
const pubKeyWithoutHash = utils.hexlify(signaturePackedWithoutHash.slice(0, 32)).substr(2);
const signatureWithoutHash = utils.hexlify(signaturePackedWithoutHash.slice(32)).substr(2);
console.log('signature.pubKey withoutHash', pubKeyWithoutHash);
console.log('signature.signature withoutHash', signatureWithoutHash);

const pub_packed_bytes= wasm.private_key_to_pubkey(yz_priv_bytes);
console.log("pub_scf",utils.hexlify(pub_packed_bytes))


const pub_xy_bytes= wasm.private_key_to_pubkey_with_xy(yz_priv_bytes);
const x= utils.hexlify(pub_xy_bytes.slice(0, 32)).substr(2);
const y= utils.hexlify(pub_xy_bytes.slice(32)).substr(2);
console.log("x",x);
console.log("y",y);

let withdraw_req = "{\"nonce\":\"1\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"1684832800\",\"position_id\":2,\"amount\":3,\"eth_address\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\"}";
let withdraw_sig = wasm.sign_withdraw(withdraw_req,"1", yz_priv);
console.log(withdraw_sig);


let transfer_req  = "{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"sender_position_id\":0,\"receiver_public_key\":\"0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":0,\"amount\":0,\"asset_id\":\"0xa\"}"
let transfer_sig = wasm.sign_transfer(transfer_req, yz_priv);
console.log(transfer_sig);

let limit_order_req = "{\"nonce\":\"1\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":3,\"amount_collateral\":4,\"amount_fee\":5,\"asset_id_synthetic\":6,\"asset_id_collateral\":\"7\",\"position_id\":8,\"is_buying_synthetic\":false}"
let limit_order_sig = wasm.sign_limit_order(limit_order_req, yz_priv);
console.log(limit_order_sig);

let liquide_req = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":1,\"amount_collateral\":2,\"amount_fee\":3,\"asset_id_synthetic\":4,\"asset_id_collateral\":\"0x5\",\"position_id\":6,\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}"
let liquide_sig = wasm.sign_liquidate(liquide_req, yz_priv);
console.log(liquide_sig);

let oracle_price_req = "{\"signer_key\":\"42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9\",\"external_price\":1,\"timestamp\":2,\"signed_asset_id\":\"0x3\"}"
let oracle_price_sig = wasm.sign_signed_oracle_price(oracle_price_req, yz_priv);
console.log(oracle_price_sig)