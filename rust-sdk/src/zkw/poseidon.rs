use crate::zkw::{Reduce, ReduceRule};
use ff::PrimeField;
use halo2_proofs::pairing::bn256::Fr;
use pairing_ce::ff::Field;
use poseidon::Poseidon;
use std::convert::TryInto;

/// Foreign functions that supports the following C code library
///
/// void poseidon(uint64_t* data, uint32_t size, uint64_t* r)
/// {
///     int i;
///     poseidon_new(size);
///     for(i=0; i<size; i=++) {
///         uint64_t* a = data[i];
///         poseidon_push(data[i]);
///     }
///     r[0] = poseidon_finalize();
///     r[1] = poseidon_finalize();
///     r[2] = poseidon_finalize();
///     r[3] = poseidon_finalize();
///     wasm_dbg(r[0]);
///     wasm_dbg(r[1]);
///     wasm_dbg(r[2]);
///     wasm_dbg(r[3]);
/// }

lazy_static::lazy_static! {
    pub static ref POSEIDON_HASHER: poseidon::Poseidon<Fr, 9, 8> = Poseidon::<Fr, 9, 8>::new(8, 63);
    // pub static ref MERKLE_HASHER: poseidon::Poseidon<Fr, 3, 2> = Poseidon::<Fr, 3, 2>::new(8, 57);
    // pub static ref POSEIDON_HASHER_SPEC: poseidon::Spec<Fr, 9, 8> = Spec::new(8, 63);
    // pub static ref MERKLE_HASHER_SPEC: poseidon::Spec<Fr, 3, 2> = Spec::new(8, 57);
}

pub struct Generator {
    pub cursor: usize,
    pub values: Vec<u64>,
}

impl Generator {
    pub fn gen(&mut self) -> u64 {
        let r = self.values[self.cursor];
        self.cursor += 1;
        if self.cursor == 4 {
            self.cursor = 0;
        }
        r
    }
}

pub fn new_reduce(rules: Vec<ReduceRule<Fr>>) -> Reduce<Fr> {
    Reduce { cursor: 0, rules }
}

pub struct PoseidonContext {
    pub hasher: Option<Poseidon<Fr, 9, 8>>,
    pub generator: Generator,
    pub buf: Vec<Fr>,
    pub fieldreducer: Reduce<Fr>,
}

impl PoseidonContext {
    pub fn default() -> Self {
        PoseidonContext {
            hasher: None,
            fieldreducer: new_reduce(vec![ReduceRule::Field(Fr::zero(), 64)]),
            buf: vec![],
            generator: Generator {
                cursor: 0,
                values: vec![],
            },
        }
    }

    pub fn poseidon_new(&mut self, new: usize) {
        self.buf = vec![];
        if new != 0 {
            self.hasher = Some(POSEIDON_HASHER.clone());
        }
    }

    pub fn poseidon_push(&mut self, v: u64) {
        self.fieldreducer.reduce(v);
        if self.fieldreducer.cursor == 0 {
            self.buf
                .push(self.fieldreducer.rules[0].field_value().unwrap())
        }
    }

    pub fn poseidon_finalize(&mut self) -> u64 {
        assert!(self.buf.len() == 8);
        if self.generator.cursor == 0 {
            let s = self.hasher.as_ref().unwrap();
            let r = s
                .clone()
                .update_exact(&self.buf.clone().try_into().unwrap());
            let dwords: Vec<u8> = r.to_repr().to_vec();
            self.generator.values = dwords
                .chunks(8)
                .map(|x| u64::from_le_bytes(x.to_vec().try_into().unwrap()))
                .collect();
            // self.hasher.as_ref().map(|s| {
            //     println!("perform hash with {:?}", self.buf);
            //     let r = s.clone().update_exact(&self.buf.clone().try_into().unwrap());
            //     let dwords: Vec<u8> = r.to_repr().to_vec();
            //     self.generator.values = dwords.chunks(8).map(|x| {
            //         u64::from_le_bytes(x.to_vec().try_into().unwrap())
            //     }).collect::<Vec<u64>>();
            // });
        }
        self.generator.gen()
    }
}

mod anposeidon {
    use std::convert::TryInto;
    // use zkwasm_rust_sdk::{PoseidonContext, POSEIDON_HASHER};
    use crate::zkw::poseidon::{PoseidonContext, POSEIDON_HASHER};
    use ff::PrimeField;
    use once_cell::sync::Lazy;
    use std::ops::DerefMut;

    static mut CONTEXT: Lazy<PoseidonContext> = Lazy::new(|| PoseidonContext::default());

    pub fn poseidon_new(x: u64) {
        let context = unsafe { &mut CONTEXT };
        context.buf = vec![];
        let new = x;
        if new != 0 {
            context.hasher = Some(POSEIDON_HASHER.clone());
        }
    }

    pub fn poseidon_push(x: u64) {
        let context = unsafe { CONTEXT.deref_mut() };
        context.fieldreducer.reduce(x);
        if context.fieldreducer.cursor == 0 {
            context
                .buf
                .push(context.fieldreducer.rules[0].field_value().unwrap())
        }
    }

    pub fn poseidon_finalize() -> u64 {
        let context = unsafe { CONTEXT.deref_mut() };
        if context.generator.cursor == 0 {
            let s = context.hasher.as_ref().unwrap();
            let r = s
                .clone()
                .update_exact(&context.buf.clone().try_into().unwrap());
            let dwords: Vec<u8> = r.to_repr().to_vec();
            context.generator.values = dwords
                .chunks(8)
                .map(|x| u64::from_le_bytes(x.to_vec().try_into().unwrap()))
                .collect::<Vec<u64>>();
            // context.hasher.as_ref().map(|s| {
            //     let r = s
            //         .clone()
            //         .update_exact(&context.buf.clone().try_into().unwrap());
            //     let dwords: Vec<u8> = r.to_repr().to_vec();
            //     context.generator.values = dwords
            //         .chunks(8)
            //         .map(|x| u64::from_le_bytes(x.to_vec().try_into().unwrap()))
            //         .collect::<Vec<u64>>();
            // });
        }
        context.generator.gen()
    }
}