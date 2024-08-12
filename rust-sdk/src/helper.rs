use crate::common::Signature;
use crate::zkw::JubjubSignature;
#[allow(dead_code)]
pub(crate) const PRI_KEY: &str =
    "0x01e1b55a539517898350ca915cbf8b25b70d9313a5ab0ff0a3466ed7799f11fe";
#[allow(dead_code)]
pub(crate) const PUB_KEY: &str =
    "0x0d4a693a09887aabea49f49a7a0968929f17b65134ab3b26201e49a43cbe7c2a";

#[allow(dead_code)]
pub(crate) fn verify_valid_sig(sig: &JubjubSignature) {
    let json = serde_json::to_string(sig).unwrap();
    let sig: Signature = serde_json::from_str(&json).unwrap();
    assert!(sig.r.len() == 66);
    assert!(sig.s.len() == 66);
}
