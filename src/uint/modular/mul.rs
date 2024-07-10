use crate::{Limb, Uint};

use super::reduction::montgomery_reduction;

#[cfg(all(
    target_os = "zkvm",
    target_vendor = "succinct",
    target_arch = "riscv32"
))]
use crate::succinct;

pub(crate) fn mul_montgomery_form<const LIMBS: usize>(
    a: &Uint<LIMBS>,
    b: &Uint<LIMBS>,
    modulus: &Uint<LIMBS>,
    mod_neg_inv: Limb,
) -> Uint<LIMBS> {
    #[cfg(all(
        target_os = "zkvm",
        target_vendor = "succinct",
        target_arch = "riscv32"
    ))]
    if LIMBS == succinct::BIGINT_WIDTH_WORDS {
        return succinct::modmul_uint_256(a, b, modulus);
    }

    let product = a.mul_wide(b);
    montgomery_reduction::<LIMBS>(&product, modulus, mod_neg_inv)
}

pub(crate) fn square_montgomery_form<const LIMBS: usize>(
    a: &Uint<LIMBS>,
    modulus: &Uint<LIMBS>,
    mod_neg_inv: Limb,
) -> Uint<LIMBS> {
    #[cfg(all(
        target_os = "zkvm",
        target_vendor = "succinct",
        target_arch = "riscv32"
    ))]
    if LIMBS == succinct::BIGINT_WIDTH_WORDS {
        return succinct::modmul_uint_256(a, a, modulus);
    }

    let product = a.square_wide();
    montgomery_reduction::<LIMBS>(&product, modulus, mod_neg_inv)
}
