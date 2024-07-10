use crate::{Uint, U128, U256};
use subtle::ConstantTimeLess;

extern "C" {
    /// Computes a big integer operation with a modulus.
    pub fn sys_bigint(
        result: *mut [u32; 8],
        op: u32,
        x: *const [u32; 8],
        y: *const [u32; 8],
        modulus: *const [u32; 8],
    );
}

pub const BIGINT_WIDTH_WORDS: usize = 8;
const OP_MULTIPLY: u32 = 0;

#[inline(always)]
pub(crate) fn modmul_uint_256<const LIMBS: usize>(
    a: &Uint<LIMBS>,
    b: &Uint<LIMBS>,
    modulus: &Uint<LIMBS>,
) -> Uint<LIMBS> {
    assert!(LIMBS == BIGINT_WIDTH_WORDS);

    let result = Uint::<LIMBS>::from_words(unsafe {
        let mut out = core::mem::MaybeUninit::<[u32; LIMBS]>::uninit();
        sys_bigint(
            out.as_mut_ptr() as *mut [u32; BIGINT_WIDTH_WORDS],
            OP_MULTIPLY,
            a.as_words().as_ptr() as *const [u32; BIGINT_WIDTH_WORDS],
            b.as_words().as_ptr() as *const [u32; BIGINT_WIDTH_WORDS],
            modulus.as_words().as_ptr() as *const [u32; BIGINT_WIDTH_WORDS],
        );
        out.assume_init()
    });
    assert!(bool::from(result.ct_lt(&modulus)));
    result
}

#[inline(always)]
pub fn mul_wide_u128(a: &U128, b: &U128) -> U256 {
    let mut a_pad = [0u32; BIGINT_WIDTH_WORDS];
    a_pad[..U128::LIMBS].copy_from_slice(a.as_words());
    let mut b_pad = [0u32; BIGINT_WIDTH_WORDS];
    b_pad[..U128::LIMBS].copy_from_slice(b.as_words());

    U256::from_words(unsafe {
        let mut out = core::mem::MaybeUninit::<[u32; BIGINT_WIDTH_WORDS]>::uninit();
        sys_bigint(
            out.as_mut_ptr(),
            OP_MULTIPLY,
            a_pad.as_ptr() as *const [u32; BIGINT_WIDTH_WORDS],
            b_pad.as_ptr() as *const [u32; BIGINT_WIDTH_WORDS],
            &[0u32; BIGINT_WIDTH_WORDS],
        );
        out.assume_init()
    })
}

#[inline(always)]
pub fn modmul_u256_denormalized(a: &U256, b: &U256, modulus: &U256) -> U256 {
    U256::from_words(unsafe {
        let mut out = core::mem::MaybeUninit::<[u32; BIGINT_WIDTH_WORDS]>::uninit();
        sys_bigint(
            out.as_mut_ptr(),
            OP_MULTIPLY,
            a.as_words(),
            b.as_words(),
            modulus.as_words(),
        );
        out.assume_init()
    })
}

#[inline(always)]
pub fn modmul_u256(a: &U256, b: &U256, modulus: &U256) -> U256 {
    let result = modmul_u256_denormalized(a, b, modulus);
    assert!(bool::from(result.ct_lt(&modulus)));
    result
}
