mod poseidon;

use crate::zkw::poseidon::{PoseidonContext, POSEIDON_HASHER};
use halo2_proofs::arithmetic::FieldExt;
use primitive_types::U256;
use serde::{Serialize, Serializer};

pub struct PoseidonHasher {
    x: u64,
    context: PoseidonContext,
}

impl PoseidonHasher {
    pub fn new() -> Self {
        let mut context = PoseidonContext::default();
        context.buf = vec![];
        let new = 1u64;
        if new != 0 {
            context.hasher = Some(POSEIDON_HASHER.clone());
        }
        PoseidonHasher {
            x: 0u64,
            context: context,
        }
    }
    pub fn update(&mut self, v: u64) {
        self.context.poseidon_push(v);
        self.x += 1;
        if self.x == 32 {
            self.context.poseidon_finalize();
            self.context.poseidon_finalize();
            self.context.poseidon_finalize();
            self.context.poseidon_finalize();
            self.context.poseidon_new(0u64 as usize);

            self.x = 0;
        }
    }
    pub fn finalize(&mut self) -> [u64; 4] {
        for _ in (self.x & 0x3)..4 {
            self.context.poseidon_push(0);
            self.x += 1;
        }
        if self.x == 32 {
            self.context.poseidon_finalize();
            self.context.poseidon_finalize();
            self.context.poseidon_finalize();
            self.context.poseidon_finalize();
            self.context.poseidon_new(0u64 as usize);
            self.x = 0;
        }

        self.context.poseidon_push(1);

        self.x += 1;
        for _ in self.x..32 {
            self.context.poseidon_push(0);
        }

        [
            self.context.poseidon_finalize(),
            self.context.poseidon_finalize(),
            self.context.poseidon_finalize(),
            self.context.poseidon_finalize(),
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BabyJubjubPoint {
    pub x: U256,
    pub y: U256,
}

pub const MODULUS: [u64; 4] = [
    0x43e1f593f0000001,
    0x2833e84879b97091,
    0xb85045b68181585d,
    0x30644e72e131a029,
];

pub fn negative_of_fr(b: &[u64; 4]) -> [u64; 4] {
    let mut borrow = 0;
    let mut a = MODULUS.clone();
    for i in 0..4 {
        if a[i] - borrow < b[i] {
            a[i] += (u64::MAX - b[i]) + 1 - borrow;
            borrow = 1
        } else {
            a[i] -= b[i] + borrow;
            borrow = 0;
        }
    }
    a
}

impl BabyJubjubPoint {
    // pub fn msm(points: Vec<(&BabyJubjubPoint, &[u64; 4])>) -> BabyJubjubPoint {
    //     let mut len = points.len();
    //     unsafe {
    //         babyjubjub_sum_new(1u64);
    //     }
    //     for (point, scalar) in points {
    //         unsafe {
    //             babyjubjub_sum_push(point.x.0[0]);
    //             babyjubjub_sum_push(point.x.0[1]);
    //             babyjubjub_sum_push(point.x.0[2]);
    //             babyjubjub_sum_push(point.x.0[3]);
    //             babyjubjub_sum_push(point.y.0[0]);
    //             babyjubjub_sum_push(point.y.0[1]);
    //             babyjubjub_sum_push(point.y.0[2]);
    //             babyjubjub_sum_push(point.y.0[3]);
    //             babyjubjub_sum_push(scalar[0]);
    //             babyjubjub_sum_push(scalar[1]);
    //             babyjubjub_sum_push(scalar[2]);
    //             babyjubjub_sum_push(scalar[3]);
    //             len -= 1;
    //             if len != 0 {
    //                 babyjubjub_sum_finalize();
    //                 babyjubjub_sum_finalize();
    //                 babyjubjub_sum_finalize();
    //                 babyjubjub_sum_finalize();
    //                 babyjubjub_sum_finalize();
    //                 babyjubjub_sum_finalize();
    //                 babyjubjub_sum_finalize();
    //                 babyjubjub_sum_finalize();
    //                 babyjubjub_sum_new(0u64);
    //             }
    //         }
    //     }
    //     unsafe {
    //         BabyJubjubPoint {
    //             x: U256([
    //                 babyjubjub_sum_finalize(),
    //                 babyjubjub_sum_finalize(),
    //                 babyjubjub_sum_finalize(),
    //                 babyjubjub_sum_finalize(),
    //             ]),
    //             y: U256([
    //                 babyjubjub_sum_finalize(),
    //                 babyjubjub_sum_finalize(),
    //                 babyjubjub_sum_finalize(),
    //                 babyjubjub_sum_finalize(),
    //             ]),
    //         }
    //     }
    // }
}

use crate::tx::packed_signature::SignatureSerde;

#[derive(Debug, Clone, PartialEq)]
pub struct JubjubSignature {
    pub sig_r: BabyJubjubPoint,
    pub sig_s: [u64; 4],
}

// 0 = c . pk + R - S . P_G that requires all points to be in the same group
// let lhs = vk.mul_scalar(&c).add(&sig_r);
// let rhs = p_g.mul_scalar(&sig_s);

const NEG_BASE: BabyJubjubPoint = BabyJubjubPoint {
    x: U256([
        5098030607081443850,
        11739138394996609992,
        7617911478965053006,
        103675969630295906,
    ]),
    y: U256([
        10973966134842004663,
        8445032247919564157,
        8665528646177973254,
        405343104476405055,
    ]),
};

impl JubjubSignature {
    // pub fn verify(&self, pk: &BabyJubjubPoint, msghash: &[u64; 4]) {
    //     unsafe {
    //         let r = BabyJubjubPoint::msm(vec![
    //             (pk, msghash),
    //             (&self.sig_r, &[1, 0, 0, 0]),
    //             (&NEG_BASE, &self.sig_s),
    //         ]);
    //         require(r.x == U256([0, 0, 0, 0]));
    //         require(r.y == U256([1, 0, 0, 0]));
    //     }
    // }
}

pub enum ReduceRule<F: FieldExt> {
    Bytes(Vec<u8>, usize),
    Field(F, usize),
    // F * shiftbits
    U64(u64),
}

impl<F: FieldExt> ReduceRule<F> {
    fn nb_inputs(&self) -> usize {
        match self {
            ReduceRule::Bytes(_, a) => *a, // a * u64
            ReduceRule::Field(_, _) => 4,  // 4 * u64
            ReduceRule::U64(_) => 1,       // 4 * u64
        }
    }
    fn reduce(&mut self, v: u64, offset: usize) {
        match self {
            ReduceRule::Bytes(ref mut x, _) => {
                let mut bytes: Vec<u8> = v.to_le_bytes().to_vec();
                x.append(&mut bytes);
            } // a * u64
            ReduceRule::Field(ref mut x, shift) => {
                let mut acc = F::from_u128(v as u128);
                for _ in 0..offset {
                    acc = acc * F::from_u128(1u128 << *shift)
                }
                *x = *x + acc
            } // 4 * u64
            ReduceRule::U64(ref mut x) => {
                *x = v;
            } // 1 * u64
        }
    }

    fn reset(&mut self) {
        match self {
            ReduceRule::Bytes(ref mut x, _) => x.clear(), // a * u64
            ReduceRule::Field(ref mut x, _shift) => *x = F::zero(), // 4 * u64
            ReduceRule::U64(ref mut x) => {
                *x = 0;
            } // 1 * u64
        }
    }

    pub fn field_value(&self) -> Option<F> {
        match self {
            ReduceRule::Bytes(_, _) => None,
            ReduceRule::Field(f, _) => Some(*f), // 4 * u64
            ReduceRule::U64(_) => None,          // 4 * u64
        }
    }
    pub fn bytes_value(&self) -> Option<Vec<u8>> {
        match self {
            ReduceRule::Bytes(b, _) => Some(b.clone()),
            ReduceRule::Field(_, _) => None, // 4 * u64
            ReduceRule::U64(_) => None,      // 4 * u64
        }
    }
    pub fn u64_value(&self) -> Option<u64> {
        match self {
            ReduceRule::Bytes(_, _) => None,
            ReduceRule::Field(_, _) => None, // 4 * u64
            ReduceRule::U64(v) => Some(*v),  // 4 * u64
        }
    }
}

pub struct Reduce<F: FieldExt> {
    pub cursor: usize,
    pub rules: Vec<ReduceRule<F>>,
}

impl<F: FieldExt> Reduce<F> {
    pub fn new(rules: Vec<ReduceRule<F>>) -> Self {
        Reduce { cursor: 0, rules }
    }
    pub fn total_len(&self) -> usize {
        self.rules.iter().fold(0, |acc, x| acc + x.nb_inputs())
    }
}

impl<F: FieldExt> Reduce<F> {
    /// take in a u64 value and update all the reduce rule accordingly
    pub fn reduce(&mut self, v: u64) {
        let mut cursor = self.cursor;
        let total = self.total_len();
        if cursor == 0 {
            for rule in self.rules.iter_mut() {
                rule.reset()
            }
        }
        for index in 0..self.rules.len() {
            if cursor >= self.rules[index].nb_inputs() {
                cursor = cursor - self.rules[index].nb_inputs();
            } else {
                self.rules[index].reduce(v, cursor);
                break;
            }
        }
        self.cursor += 1;
        if self.cursor == total {
            self.cursor = 0;
        }
    }
}