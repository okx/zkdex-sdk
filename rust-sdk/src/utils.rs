use franklin_crypto::alt_babyjubjub::fs::{Fs, FsRepr};
use franklin_crypto::alt_babyjubjub::AltJubjubBn256;
use franklin_crypto::eddsa::Signature;
use franklin_crypto::jubjub::edwards::Point;
use franklin_crypto::jubjub::Unknown;
use franklin_crypto::{
    bellman::{pairing::ff::PrimeField},
};

use pairing_ce::bn256::Bn256;
use pairing_ce::ff::PrimeFieldRepr;
use primitive_types::U256;
use zkdex_utils::tx::packed_public_key::u256_to_fr;
use zkdex_utils::tx::packed_signature::{signature_from_rs, PackedSignature, SignatureOriginal};
use zkdex_utils::tx::{le_to_u256, JUBJUB_PARAMS};
use zkdex_utils::tx::baby_jubjub::*;
use zkdex_wasm::HashType;
use primitive_types::H256;


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

pub fn u256_to_h256(u: U256) -> H256 {
    let mut h = [0u8; 32];
    u.to_little_endian(&mut h[..]);
    H256(h)
}