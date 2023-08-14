use bitvec::order::Msb0;
use bitvec::view::BitView;

pub fn vec_u8_to_vec_bool(v: &Vec<u8>) -> Vec<bool> {
    let bits = v.view_bits::<Msb0>();

    bits.iter().map(|b| *b).collect()
}
