use franklin_crypto::alt_babyjubjub::fs::{Fs, FsRepr};
use franklin_crypto::alt_babyjubjub::AltJubjubBn256;
use franklin_crypto::bellman::Field;
use franklin_crypto::eddsa::Signature;
use franklin_crypto::jubjub::edwards::Point;
use franklin_crypto::jubjub::Unknown;
use franklin_crypto::{
    bellman::{pairing::ff::PrimeField, BitIterator},
    circuit::multipack,
    eddsa::PublicKey,
    rescue::rescue_hash,
};

use crate::RESCUE_PARAMS;
use num::Zero;
use num_bigint::BigInt;
use pairing_ce::bn256::Bn256;
use pairing_ce::ff::PrimeFieldRepr;
use primitive_types::U256;
use zkdex_utils::tx::packed_public_key::u256_to_fr;
use zkdex_utils::tx::packed_signature::{signature_from_rs, PackedSignature, SignatureOriginal};
use zkdex_utils::tx::{le_to_u256, JUBJUB_PARAMS};
use zkdex_wasm::HashType;
use zkwasm_rust_sdk::JubjubSignature;

use crate::tx::u256_to_h256;
use crate::{Engine, Fr};

const PAD_MSG_BEFORE_HASH_BITS_LEN: usize = 736;
const NEW_PUBKEY_HASH_WIDTH: usize = 160;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn bytes_into_be_bits(bytes: &[u8]) -> Vec<bool> {
    let mut bits = Vec::with_capacity(bytes.len() * 8);
    for byte in bytes {
        let mut temp = *byte;
        for _ in 0..8 {
            bits.push(temp & 0x80 == 0x80);
            temp <<= 1;
        }
    }
    bits
}

pub fn pack_bits_into_bytes(bits: &[bool]) -> Vec<u8> {
    let mut message_bytes: Vec<u8> = Vec::with_capacity(bits.len() / 8);
    let byte_chunks = bits.chunks(8);
    for byte_chunk in byte_chunks {
        let mut byte = 0u8;
        for (i, bit) in byte_chunk.iter().enumerate() {
            if *bit {
                byte |= 1 << i;
            }
        }
        message_bytes.push(byte);
    }
    message_bytes
}

pub fn pack_bits_into_bytes_le(bits: &[bool]) -> Vec<u8> {
    let mut message_bytes: Vec<u8> = Vec::with_capacity(bits.len() / 8);
    let byte_chunks = bits.chunks(8);
    for byte_chunk in byte_chunks {
        let mut byte = 0u8;
        for (i, bit) in byte_chunk.iter().rev().enumerate() {
            if *bit {
                byte |= 1 << i;
            }
        }
        message_bytes.push(byte);
    }
    message_bytes
}

pub fn append_le_fixed_width(content: &mut Vec<bool>, x: &Fr, width: usize) {
    let mut token_bits: Vec<bool> = BitIterator::new(x.into_repr()).collect();
    token_bits.reverse();
    token_bits.resize(width, false);
    content.extend(token_bits);
}

pub fn pub_key_hash(pub_key: &PublicKey<Engine>) -> Vec<u8> {
    let (pub_x, pub_y) = pub_key.0.into_xy();
    let pub_key_hash = rescue_hash_elements(&[pub_x, pub_y]);
    let mut pub_key_hash_bits = Vec::with_capacity(NEW_PUBKEY_HASH_WIDTH);
    append_le_fixed_width(&mut pub_key_hash_bits, &pub_key_hash, NEW_PUBKEY_HASH_WIDTH);
    let mut bytes = pack_bits_into_bytes(&pub_key_hash_bits);
    bytes.reverse();
    bytes
}

fn rescue_hash_fr(input: Vec<bool>) -> Fr {
    RESCUE_PARAMS.with(|params| {
        let packed = multipack::compute_multipacking::<Engine>(&input);
        let sponge_output = rescue_hash::<Engine>(params, &packed);
        assert_eq!(sponge_output.len(), 1, "rescue hash problem");
        sponge_output[0]
    })
}

fn rescue_hash_elements(input: &[Fr]) -> Fr {
    RESCUE_PARAMS.with(|params| {
        let sponge_output = rescue_hash::<Engine>(params, input);
        assert_eq!(sponge_output.len(), 1, "rescue hash problem");
        sponge_output[0]
    })
}

pub fn rescue_hash_tx_msg(msg: &[u8]) -> Vec<u8> {
    let mut msg_bits = bytes_into_be_bits(msg);
    assert!(msg_bits.len() <= PAD_MSG_BEFORE_HASH_BITS_LEN);
    msg_bits.resize(PAD_MSG_BEFORE_HASH_BITS_LEN, false);
    let hash_fr = rescue_hash_fr(msg_bits);
    let mut hash_bits = Vec::new();
    append_le_fixed_width(&mut hash_bits, &hash_fr, 256);
    pack_bits_into_bytes(&hash_bits)
}

fn get_bits_le_fixed(fr: &Fr, size: usize) -> Vec<bool> {
    let mut bits: Vec<bool> = Vec::with_capacity(size);
    let repr = fr.into_repr();
    let repr: &[u64] = repr.as_ref();
    let n = std::cmp::min(repr.len() * 64, size);
    for i in 0..n {
        let part = i / 64;
        let bit = i - (64 * part);
        bits.push(repr[part] & (1 << bit) > 0);
    }
    let n = bits.len();
    bits.extend((n..size).map(|_| false));
    bits
}

pub fn rescue_hash_orders(msg: &[u8]) -> Vec<u8> {
    assert_eq!(msg.len(), 178);
    let msg_bits = bytes_into_be_bits(msg);
    let hash_fr = rescue_hash_fr(msg_bits);
    let hash_bits = get_bits_le_fixed(&hash_fr, 248);
    pack_bits_into_bytes_le(&hash_bits)
}

pub fn fr_from_bigint(num: &BigInt) -> Fr {
    if num > &BigInt::zero() {
        Fr::from_str(&num.to_string()).unwrap()
    } else {
        let mut num = Fr::from_str(&(-num).to_string()).unwrap();
        num.negate();
        num
    }
}

pub fn hash_type_to_string_with_0xprefix(hash: HashType) -> String {
    let mut be = [0u8; 32];
    hash.to_big_endian(&mut be);
    format!("0x{:x}", primitive_types::H256(be))
}

pub fn jubjub_to_json(sig: &JubjubSignature) -> String {
    let mut r = [0u8; 32];
    let r_point = point_from_xy(&sig.sig_r.x, &sig.sig_r.y);
    r_point.write(r.as_mut()).unwrap();

    let r = le_to_u256(&r);
    let s = U256(sig.sig_s);
    let sign = SignatureOriginal { r, s };
    let json = serde_json::to_string(&sign).unwrap();
    json
}

pub fn jubjub_signature_to_packed_signature(value: JubjubSignature) -> PackedSignature {
    let r = point_from_xy(&value.sig_r.x, &value.sig_r.y);
    let s = u256_to_h256(U256(value.sig_s)).0;
    let mut fspr = FsRepr::default();
    fspr.read_le(&s[..]).unwrap();
    let s = Fs::from_repr(fspr).unwrap();
    PackedSignature {
        0: Signature { r: r, s: s },
    }
}

pub fn jubjub_signature_from_str(r: &str, s: &str) -> JubjubSignature {
    let r_str = r.trim_start_matches("0x").trim_start_matches("0X");
    let r = U256::from_str_radix(r_str, 16).unwrap();

    let s_str = s.trim_start_matches("0x").trim_start_matches("0X");
    let s = U256::from_str_radix(s_str, 16).unwrap();
    signature_from_rs(&r, &s)
}

pub fn point_from_xy(x: &U256, y: &U256) -> Point<Bn256, Unknown> {
    let x = u256_to_fr(x).unwrap();
    let y = u256_to_fr(y).unwrap();

    Point::from_xy(x, y, &JUBJUB_PARAMS as &AltJubjubBn256).unwrap()
}
