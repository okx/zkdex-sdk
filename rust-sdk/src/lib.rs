#![feature(test)]
extern crate test as other_test;

/// # The rust-sdk
/// The rust-sdk crate provides a set of functions to interact with the ZKDex.
use std::convert::TryFrom;
use std::str::FromStr;

use anyhow::{Error, Result};
pub use convert::*;
use ethers::abi::{encode_packed, AbiEncode, Token};
pub use franklin_crypto::bellman::pairing::bn256::{Bn256 as Engine, Fr};
use franklin_crypto::rescue::bn256::Bn256RescueParams;
use franklin_crypto::{
    alt_babyjubjub::{fs::FsRepr, AltJubjubBn256, FixedGenerators},
    bellman::pairing::ff::{PrimeField, PrimeFieldRepr},
    eddsa::PublicKey,
    jubjub::JubjubEngine,
};
use num::Integer;
use num_bigint::BigInt;
use num_traits::Num;
use primitive_types::U256;
use serde::{Deserialize, Serialize};
use sha2::{Digest as tDigest, Sha256};
use sha3::Digest;
use sha3::Keccak256;

use crate::crypto::convert::FeConvert;
use crate::crypto::packed_public_key::{
    convert_to_pubkey, private_key_from_string, public_key_from_private, PackedPublicKey,
};
use crate::crypto::packed_signature::{get_r_from_xy, PackedSignature};
use crate::crypto::sign::TxSignature;
use crate::felt::LeBytesConvert;
use crate::types::{Fs, HashType};
use crate::zkw::{BabyJubjubPoint, JubjubSignature};
pub use perpetual::*;
pub use serde_wrapper::*;
pub use spot::*;
pub use unified::*;

pub mod common;
mod constant;
mod convert;

mod crypto;
mod hash;
pub(crate) mod helper;
pub mod java_bridge;
pub mod javascript_bridge;
mod perpetual;
pub(crate) mod serde_wrapper;
pub mod spot;
mod types;
pub mod unified;
mod utils;
pub mod zkw;

thread_local! {
    pub static JUBJUB_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
    pub static RESCUE_PARAMS: Bn256RescueParams = Bn256RescueParams::new_checked_2_into_1();
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Generate a private key from a seed, the seed should be a byte array and its length should be greater than 32.
///
/// # Examples
///
/// ```
/// use zkdex_sdk::private_key_from_seed;
/// let seed = "hi welcome to zkdex, this is a seed for private key generation";
/// let pri_key = private_key_from_seed(seed.as_bytes());
/// assert!(pri_key.is_ok());
/// ```
pub fn private_key_from_seed(seed: &[u8]) -> Result<String> {
    if seed.len() < 32 {
        return Err(Error::msg("seed is too short"));
    };

    let sha256_bytes = |input: &[u8]| -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(input);
        hasher.result().to_vec()
    };

    let mut effective_seed = sha256_bytes(seed);

    loop {
        let raw_priv_key = sha256_bytes(&effective_seed);
        let mut fs_repr = FsRepr::default();
        fs_repr
            .read_be(&raw_priv_key[..])
            .expect("failed to read raw_priv_key");
        if Fs::from_repr(fs_repr).is_ok() {
            return Ok("0x".to_string() + &hex::encode(raw_priv_key));
        } else {
            effective_seed = raw_priv_key;
        }
    }
}

/// Derive a public key with separate x and y coordinates from a private key.
///
/// # Examples
///
/// ```
/// use zkdex_sdk::{private_key_from_seed, private_key_to_pubkey_xy};
/// let private_key = private_key_from_seed("hi welcome to zkdex, this is a seed for private key generation".as_bytes()).unwrap();
/// let xy = private_key_to_pubkey_xy(&private_key);
/// assert!(xy.is_ok());
/// ```
pub fn private_key_to_pubkey_xy(private_key: &str) -> Result<(String, String)> {
    let pri_key = private_key_from_string(private_key)?;
    let packed_pk: PackedPublicKey = public_key_from_private(&pri_key);
    Ok(pub_key_to_xy(&packed_pk.to_string())?)
}

/// Convert a public key to separate x and y coordinates.
///
/// # Examples
/// ```
/// use zkdex_sdk::pub_key_to_xy;
/// let pub_key = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
/// let xy = pub_key_to_xy(pub_key);
/// assert!(xy.is_ok());
/// ```
pub fn pub_key_to_xy(pub_key: &str) -> Result<(String, String)> {
    let pub_key = pub_key.trim_start_matches("0x").trim_start_matches("0X");
    let packed_pk = PackedPublicKey::try_from(pub_key)?;

    let jubjub_pk: BabyJubjubPoint = packed_pk.into();
    let mut x_point = [0; 32];
    jubjub_pk.x.to_big_endian(&mut x_point);

    Ok((
        "0x".to_owned() + &pub_key.to_string(),
        "0x".to_owned() + &hex::encode(x_point.to_vec()),
    ))
}

/// Sign a msg with a private key.
/// This is a basic function to sign a message with a private key.
/// # Examples
/// ```
/// use zkdex_sdk::{private_key_from_seed, sign};
///
/// let private = private_key_from_seed("hi welcome to zkdex, this is a seed for private key generation".as_bytes()).unwrap();
/// let msg = "0x08a09b19adaa35815065dffcc4b5e0ee75f54660eb474c5932929b96c0ff15c9";
/// let sig = sign(&private, msg);
/// assert!(sig.is_ok());
/// ```
pub fn sign(private_key: &str, msg: &str) -> Result<JubjubSignature> {
    let hash = HashType::from_str(msg)?;
    let private_key = private_key_from_string(private_key)?;
    let (sig, _) = TxSignature::sign_msg(&private_key, hash.as_le_bytes());
    Ok(sig.into())
}

/// Verify a seperated signature: r,s with a seperated public key: x,y
/// # Examples
/// ```
/// use zkdex_sdk::verify_signature;
/// let sig_r = "0x2e39e39381ac5e962650072a8936b99716fc0b3fda124f59ef62066301fd0749";
/// let sig_s = "0x37fd915bf958893ed35132a91b98fc4fcd7821c9fe784057bbc85d8fc5e7d4f";
/// let pub_key_x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
/// let pub_key_y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";
/// let msg = "0x08a09b19adaa35815065dffcc4b5e0ee75f54660eb474c5932929b96c0ff15c9";
/// let ret = verify_signature(sig_r, sig_s, pub_key_x, pub_key_y, msg);
/// assert!(ret.is_ok());
/// assert!(ret.unwrap());
/// ```
pub fn verify_signature(
    sig_r: &str,
    sig_s: &str,
    pub_key_x: &str,
    _pub_key_y: &str,
    msg: &str,
) -> Result<bool> {
    let sig = JubjubSignature::from_str(sig_r, sig_s);
    let sig = PackedSignature::from(sig);
    let msg = HashType::from_str(msg)?;
    let packed_pk = PackedPublicKey::try_from(pub_key_x)?;
    let jubjub_pk: BabyJubjubPoint = packed_pk.into();
    let pk = convert_to_pubkey(&jubjub_pk.x, &jubjub_pk.y)?;
    Ok(sig.verify(&pk, msg.as_le_bytes()))
}

/// Verify Jubjub signature internally.
pub(crate) fn verify_jubjub_signature(
    sig: JubjubSignature,
    pub_key: &str,
    msg: &str,
) -> Result<bool> {
    let sig = PackedSignature::from(sig);
    let msg = HashType::from_str(msg)?;
    let packed_pk = PackedPublicKey::try_from(pub_key)?;
    let jubjub_pk: BabyJubjubPoint = packed_pk.into();
    let pk = convert_to_pubkey(&jubjub_pk.x, &jubjub_pk.y)?;
    Ok(sig.verify(&pk, msg.as_le_bytes()))
}

/// Check public key x and y is on the curve.
/// # Examples
/// ```
///  use zkdex_sdk::is_on_curve;
/// let x = "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a";
///  let y = "0x0a3b966094be6c8981a22359df81f7fcdd50ac725401e3fc5872c780d158fb18";
/// let ret = is_on_curve(x, y);
///  assert!(ret.is_ok());
/// assert!(ret.unwrap());
/// ```
pub fn is_on_curve(x: &str, y: &str) -> Result<bool> {
    let x = trim_0x(x);
    let y = trim_0x(y);
    let (x1, y1) = pub_key_to_xy(x)?;
    let x1 = trim_0x(x1.as_str());
    let y1 = trim_0x(y1.as_str());
    Ok(x1 == x && y1 == y)
}

/// This is special Signature, which contains x, r, s, and public key x, y.
#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct L2Signature {
    pub x: String,
    pub y: String,
    pub s: String,
    pub pk_x: String,
    pub pk_y: String,
}

/// Sign a message with a private key then return a L2Signature.
/// # Examples
/// ```
///  use zkdex_sdk::l2_sign;
/// let msg = "0x196cdf49e6d3f3614fdba8e3459fef498685b88627b80035c62beaa7ca056eea";
///  let pri_key = "0x03f2d0a8ec58aac5ad28ac9bbc76a43c2f40c167885c9117b5863545dd2471f3";
///  let sig = l2_sign(msg, pri_key);
///  assert!(sig.is_ok());
/// ```
pub fn l2_sign(msg: &str, private_key: &str) -> Result<L2Signature> {
    let msg = msg.trim_start_matches("0x").trim_start_matches("0X");
    let private_key = private_key_from_string(private_key)?;
    let msg = HashType::from_str(msg)?;
    let (sig, packed_pk) = TxSignature::sign_msg(&private_key, msg.as_le_bytes());
    let p_g = FixedGenerators::SpendingKeyGenerator;
    let pk = PublicKey::from_private(&private_key, p_g, &AltJubjubBn256::new());
    let (pk_x, _) = pk.0.into_xy();
    let (x, y) = sig.signature.0.r.into_xy();
    Ok(L2Signature {
        x: "0x".to_owned() + &x.to_hex(),
        y: "0x".to_owned() + &y.to_hex(),
        s: "0x".to_owned() + &sig.signature.0.s.to_hex(),
        pk_x: packed_pk.to_string(),
        pk_y: "0x".to_owned() + &pk_x.to_hex(),
    })
}

/// Verify a L2Signature.
/// # Examples
/// ```
/// use zkdex_sdk::{l2_sign, l2_verify};
/// let msg = "0x196cdf49e6d3f3614fdba8e3459fef498685b88627b80035c62beaa7ca056eea";
///  let pri_key = "0x03f2d0a8ec58aac5ad28ac9bbc76a43c2f40c167885c9117b5863545dd2471f3";
///  let s = l2_sign(msg, pri_key).unwrap();
///  assert!(l2_verify(&s.x, &s.y, &s.s, &s.pk_x, &s.pk_y, msg).unwrap());
/// ```
pub fn l2_verify(x: &str, y: &str, s: &str, pk_x: &str, pk_y: &str, msg: &str) -> Result<bool> {
    let x = trim_0x(x);
    let y = trim_0x(y);
    let r = get_r_from_xy(
        &U256::from_str_radix(x, 16).unwrap(),
        &U256::from_str_radix(y, 16).unwrap(),
    );
    let s = trim_0x(s);
    verify_signature(&format!("0x{:064x}", r), s, pk_x, pk_y, msg)
}

/// Sign for register a eth address.
/// # Examples
/// ```
/// use zkdex_sdk::{private_key_from_seed, sign_eth_address};
/// let chain_id = "11155111";
///  let contract_address = "0x4b551A084cDdB1a5355Ce17155669A5ce6e94C4E";
///  let address = "0x505cec5b6c108dbf289c935802d6f8b53b5ae5b2";
///  let pub_key = "0x864d63b304b5635579771c0864def9bbc166ae5b1f39a894998ef350f6c521ac";
///  let pri_key = private_key_from_seed("hello zkdex ggggggggggggggggggggggggg".as_bytes()).unwrap();
///  let sig = sign_eth_address(chain_id, contract_address, address, pub_key, &pri_key);
///  assert!(sig.is_ok());
/// ```
pub fn sign_eth_address(
    chain_id: &str,
    contract_address: &str,
    address: &str,
    pub_key: &str,
    private_key: &str,
) -> Result<String> {
    let t1 = Token::String("UserRegistration:".to_string());
    let t2 = Token::FixedBytes(U256::from_str_radix(chain_id, 10).unwrap().encode());
    let t3 = Token::Address(contract_address.parse().unwrap());
    let t4 = Token::Address(address.parse().unwrap());

    //let t3 = Token::Uint(U256::from_str_radix(pub_key, 16).unwrap());

    let t5 = Token::FixedBytes(U256::from_str_radix(pub_key, 16).unwrap().encode());
    let data = encode_packed(&[t1, t2, t3, t4, t5]).unwrap();
    let result = Keccak256::digest(data.as_slice());
    let max = BigInt::from_str(
        "21888242871839275222246405745257275088548364400416034343698204186575808495617",
    )
    .unwrap();
    let hash = BigInt::from_str_radix(&hex::encode(result), 16)
        .unwrap()
        .mod_floor(&max)
        .to_str_radix(16);
    let sig = l2_sign(&hash, private_key)?;
    let sig = sig.x
        + sig.y.trim_start_matches("0x")
        + sig.s.trim_start_matches("0x")
        + sig.pk_y.trim_start_matches("0x");
    Ok(sig)
}

#[cfg(test)]
mod test {

    use pairing_ce::bn256::Fr;

    use crate::common::Signature;
    use crate::crypto::convert::FeConvert;
    use crate::crypto::packed_public_key::{private_key_from_string, public_key_from_private};
    use crate::helper::{verify_valid_sig, PRI_KEY};
    use crate::perpetual::{hash_limit_order, sign_limit_order};
    use crate::{
        is_on_curve, l2_sign, l2_verify, private_key_from_seed, private_key_to_pubkey_xy,
        pub_key_to_xy, reverse_hex, sign, sign_eth_address, verify_jubjub_signature,
        verify_signature, L2Signature,
    };

    #[test]
    pub fn test_verify_signature() {
        let sigr = "0x2e39e39381ac5e962650072a8936b99716fc0b3fda124f59ef62066301fd0749";
        let sigs = "0x37fd915bf958893ed35132a91b98fc4fcd7821c9fe784057bbc85d8fc5e7d4f";
        let msg = "0x08a09b19adaa35815065dffcc4b5e0ee75f54660eb474c5932929b96c0ff15c9";
        let err_msg = "0x01817ed5bea1d0082c0fbe18edb06c15f52e2bb98c2b92f36d1a5ab082f1a520";
        let pub_x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        let pub_y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";
        let ret = verify_signature(sigr, sigs, pub_x, pub_y, msg).unwrap();
        assert!(ret);
        let ret = verify_signature(sigr, sigs, pub_x, pub_y, err_msg).unwrap();
        assert!(!ret);
    }

    #[test]
    #[should_panic]
    pub fn test_verify_signature_with_err_sig() {
        let sigr = "0x2e39e39381ac5e962650072a893b99716fc0b3fda124f";
        let sigs = "0x37fd915bf958893ed35132a91b98fc4fcd7821c9fe784057bbc85d8fc5e7d4f";
        let msg = "0x08a09b19adaa35815065dffcc4b5e0ee75f54660eb474c5932929b96c0ff15c9";
        let pub_x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        let pub_y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7";
        let ret = verify_signature(sigr, sigs, pub_x, pub_y, msg).unwrap();
        assert!(ret)
    }

    #[test]
    fn test_private_key_to_pubkey_xy() {
        let (x, y) = private_key_to_pubkey_xy(PRI_KEY).unwrap();
        assert!(x.len() == 66);
        assert!(y.len() == 66);
        println!("x: {}, y: {}", x, y);
    }

    #[test]
    fn test_private_key_from_seed() {
        let seed = "hello world good life 996 very nice";
        let pri_key = private_key_from_seed(seed.as_bytes()).unwrap();
        assert!(pri_key.len() == 66);
    }

    #[test]
    #[should_panic(expected = "seed is too short")]
    fn test_private_key_from_empty_seed() {
        let seed = "";
        let pri_key = private_key_from_seed(seed.as_bytes()).unwrap();
        assert!(pri_key.len() == 66);
    }

    #[test]
    fn test_is_on_curve() {
        let x = "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a";
        let y = "0x0a3b966094be6c8981a22359df81f7fcdd50ac725401e3fc5872c780d158fb18";
        assert!(is_on_curve(x, y).unwrap());

        let pri = "0x0376204fa0b554ee3d8a03c6ccdb73f7b98d1965fbeaa3a9f88723669a23893f";
        let (x, y) = private_key_to_pubkey_xy(pri).unwrap();
        assert!(is_on_curve(&x, &y).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_is_on_curve_with_err_x() {
        let x = "0x0d93a09887aaba49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a";
        let y = "0x0a966094be6c8981a22359df81f7fcdd50ac725401e3fc5872c780d158fb18";
        let ret = is_on_curve(&x, &y).unwrap();
        assert!(ret == false);
    }

    #[test]
    fn test_pub_key_to_xy() {
        let pk = "8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa";
        let (x, y) = pub_key_to_xy(pk).unwrap();
        assert!(x.len() == 66);
        assert!(y.len() == 66);
    }

    #[test]
    #[should_panic]
    fn test_pub_key_to_xy_with_err_pub_key() {
        let pk = "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7";
        let (x, y) = pub_key_to_xy(pk).unwrap();
        assert!(x.len() == 66);
        assert!(y.len() == 66);
    }

    #[test]
    fn test_sign() {
        let pri = "0x028bfbb9eafdacf8d76c1c35c1ed25979480d3e46d8bb391778f0fc9d40aaf70";
        let msg = "0x01b9c04067307822ea5909e9c86163128a76afbc90de47c77705cd4a4f33533f";
        let sig = sign(pri, msg).unwrap();
        let json = serde_json::to_string(&sig).unwrap();
        let sig_json: Signature = serde_json::from_str(&json).unwrap();
        assert!(sig_json.r.len() == 66);
        assert!(sig_json.s.len() == 66);
        let pk = public_key_from_private(&private_key_from_string(pri).unwrap());
        assert!(verify_jubjub_signature(sig, pk.to_string().as_str(), msg).unwrap())
    }

    #[test]
    fn test_reverse_hex() {
        let num = "12ba9000";
        println!("{:?}", reverse_hex(num).unwrap());
    }

    #[test]
    fn test_sign_trade_order() {
        let json1 = r#"
        {
      "nonce": "1",
      "public_key": "0daed291535086c7569618ec99b090c220ac63add8ab019690c3ef3b40ca970a",
      "expiration_timestamp": "3608164305",
      "signature": {
        "r": "0xaff60be77ca88a6bd9f25c06ee58e80cc27567022cb75a39d8de9bfad32b8f20",
        "s": "0xc876eb02c24f639e47383e5da2a24a0fea9da2201077ba2fe75ba88c2d242304"
      },
      "amount_synthetic": "10000000000",
      "amount_collateral": "30000000000",
      "amount_fee": "0",
      "asset_id_synthetic": "0x4254432d3130000000000000000000",
      "asset_id_collateral": "0xa21edc9d9997b1b1956f542fe95922518a9e28ace11b7b2972a1974bf5971f",
      "position_id": "10026",
      "is_buying_synthetic": true
    }
    "#;

        let json2 = r#"
        {
      "nonce": "0",
      "public_key": "0x9bb04dba1329711e145d387f71926fb2b81496c72210d53588200a954dbb443f",
      "expiration_timestamp": "3608164305",
      "signature": {
        "r": "0x532a22fcdcf55ea7badfca68c1f04c2cd1eeaa1020d69c0c589aafd429fe040f",
        "s": "0xad4c23dd453a9b313c451596d3797c9fbcdeb1e2fd654c4e24fe3a56350d7900"
      },
      "amount_synthetic": "10000000000",
      "amount_collateral": "30000000000",
      "amount_fee": "0",
      "asset_id_synthetic": "0x4254432d3130000000000000000000",
      "asset_id_collateral": "0xa21edc9d9997b1b1956f542fe95922518a9e28ace11b7b2972a1974bf5971f",
      "position_id": "10027",
      "is_buying_synthetic": false
    }
    "#;

        let json_arr = vec![json1, json2];
        let pri_arr = vec![
            "0279df312299a1400f0438e38a46432136306c531359a5edd359ae6556adf6cc",
            "042f82c4c360326263672ae3feefd4509201989e0660c0f625f47af81c975fc8",
        ];

        for (i, v) in json_arr.into_iter().enumerate() {
            let pk =
                public_key_from_private(&private_key_from_string(pri_arr[i]).unwrap()).to_string();
            let sig = sign_limit_order(v, pri_arr[i]).unwrap();
            let hash = hash_limit_order(v).unwrap();
            verify_valid_sig(&sig);
            assert!(verify_jubjub_signature(sig, pk.as_str(), hash.as_str()).unwrap());
        }
    }

    #[test]
    fn test_hex_encode() {
        let a = [1u8; 32];
        let s = hex::encode(a.to_vec());
        assert!(s.len() == 64);

        let fr = Fr::from_hex("0x1111").unwrap();
        assert!(fr.to_hex().len() == 64);
    }

    #[test]
    pub fn test_l1_sign() {
        let msg = "0x196cdf49e6d3f3614fdba8e3459fef498685b88627b80035c62beaa7ca056eea";
        let priv_key = "0x03f2d0a8ec58aac5ad28ac9bbc76a43c2f40c167885c9117b5863545dd2471f3";
        let s = l2_sign(msg, priv_key).unwrap();
        let expected = L2Signature {
            x: "0x062b74e4bde7c5655093bcfd717b2be2757fc7c85f2b5fdc0f43820df2ce510a".to_string(),
            y: "0x124c1159c6164b8f80348f23a39ff79af229ecb2f00e806e60798601607c4595".to_string(),
            s: "0x04f89ebc83800e89f19e3501562793e2d9097b921ee0759b5f37017b993238c4".to_string(),
            pk_x: "0x96c4d93a49c8159e27542601ba19fdfce52b3e9b43dafaefe9aa9cd32efded86".to_string(),
            pk_y: "0x0cc8a68b8dba85bd5418e308b34439ddffca3a0f6589a32f02adf60da6e73f55".to_string(),
        };

        assert!(l2_verify(&s.x, &s.y, &s.s, &s.pk_x, &s.pk_y, msg).unwrap() == true);
        assert!(s == expected);
    }

    #[test]
    pub fn test_eth_address_sign() {
        let chain_id = "11155111";
        let contract_address = "0x4b551A084cDdB1a5355Ce17155669A5ce6e94C4E";
        let address = "0x505cec5b6c108dbf289c935802d6f8b53b5ae5b2";
        let pub_key = "0x864d63b304b5635579771c0864def9bbc166ae5b1f39a894998ef350f6c521ac";
        let pri_key =
            private_key_from_seed("hello zkdex ggggggggggggggggggggggggg".as_bytes()).unwrap();
        let sig = sign_eth_address(chain_id, contract_address, address, pub_key, &pri_key);
        assert!(sig.is_ok());
        assert_eq!(sig.unwrap(), "0x0a503625f4d8402e9e252f4a77e9d2ac5e6e347f2689d31a39291e221f1e4cfb1a9bc2f5d8a70cba5b7bbdc6da09f948f22bafecabc6f8d2f0f23078d603322105eccbe6b4b8ada0b52f0779709fb2bb077251a19ce4d5d4c7a705974a4175932a8b55b82014ae69934fe86168fed0670d971ac72ee48ad23a529cad0c941237");

        let address = "0x6adb25ce1b29cd004fdedf40ec5c8f51e33f11ad";
        let pub_key = "0x00019dd2c8149fae983deac2ce3917476080aaadc420d560a91e56280a576b66";
        let pri_key = "0x05b82dd4f0325bf5fe7cc45ed2e8e8b47388d905f6b1d87c437f9732197425c4";
        let sig = sign_eth_address(chain_id, contract_address, address, pub_key, pri_key);
        assert!(sig.is_ok());
        assert_eq!(sig.unwrap(), "0x03b213c47af8c4f8bd4d72b7ac51e92058d9b1be1fc5bfdf1ee8abb461ec90cc2173babf96b1d41834dbf84decc645bd19fbd74610422b08b4f0632bfae301d804c71a60d4667ffec3c33f48ab50dabfbfa2fbb71bc663a1c8794fc11fb231e50fec8b35377b0f9bef295855de35e9d09e20379704d89f091f8343647490f68b");
    }
}
