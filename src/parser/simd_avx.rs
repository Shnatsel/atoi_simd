use super::*;
use crate::simd_sse_avx::*;

#[cfg(target_pointer_width = "64")]
impl ParserPos<usize> for usize {
    #[inline(always)]
    fn atoi_simd_parse_pos(s: &[u8]) -> Result<usize, AtoiSimdError> {
        parse_simd_checked_u64(s).map(|v| v as usize)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid_pos(s: &[u8]) -> Result<(usize, usize), AtoiSimdError> {
        parse_simd_u64(s).map(|(v, i)| (v as usize, i))
    }
}
#[cfg(target_pointer_width = "64")]
impl Parser<usize> for usize {}

#[cfg(target_pointer_width = "64")]
impl ParserPos<isize> for isize {
    #[inline(always)]
    fn atoi_simd_parse_pos(s: &[u8]) -> Result<isize, AtoiSimdError> {
        parse_simd_checked_i64(s).map(|v| v as isize)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid_pos(s: &[u8]) -> Result<(isize, usize), AtoiSimdError> {
        parse_simd_i64(s).map(|(v, i)| (v as isize, i))
    }
}

#[cfg(target_pointer_width = "64")]
impl ParserNeg<isize> for isize {
    #[inline(always)]
    fn atoi_simd_parse_neg(s: &[u8]) -> Result<isize, AtoiSimdError> {
        parse_simd_checked_i64_neg(s).map(|v| v as isize)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid_neg(s: &[u8]) -> Result<(isize, usize), AtoiSimdError> {
        parse_simd_i64_neg(s).map(|(v, i)| (v as isize, i))
    }
}

#[cfg(target_pointer_width = "64")]
impl Parser<isize> for isize {
    #[inline(always)]
    fn atoi_simd_parse(s: &[u8]) -> Result<isize, AtoiSimdError> {
        atoi_simd_parse_signed(s)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid(s: &[u8]) -> Result<(isize, usize), AtoiSimdError> {
        atoi_simd_parse_until_invalid_signed(s)
    }
}

impl ParserPos<u64> for u64 {
    #[inline(always)]
    fn atoi_simd_parse_pos(s: &[u8]) -> Result<u64, AtoiSimdError> {
        parse_simd_checked_u64(s)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid_pos(s: &[u8]) -> Result<(u64, usize), AtoiSimdError> {
        parse_simd_u64(s)
    }
}
impl Parser<u64> for u64 {}

impl ParserPos<i64> for i64 {
    #[inline(always)]
    fn atoi_simd_parse_pos(s: &[u8]) -> Result<i64, AtoiSimdError> {
        parse_simd_checked_i64(s)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid_pos(s: &[u8]) -> Result<(i64, usize), AtoiSimdError> {
        parse_simd_i64(s)
    }
}

impl ParserNeg<i64> for i64 {
    #[inline(always)]
    fn atoi_simd_parse_neg(s: &[u8]) -> Result<i64, AtoiSimdError> {
        parse_simd_checked_i64_neg(s)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid_neg(s: &[u8]) -> Result<(i64, usize), AtoiSimdError> {
        parse_simd_i64_neg(s)
    }
}

impl Parser<i64> for i64 {
    #[inline(always)]
    fn atoi_simd_parse(s: &[u8]) -> Result<i64, AtoiSimdError> {
        atoi_simd_parse_signed(s)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid(s: &[u8]) -> Result<(i64, usize), AtoiSimdError> {
        atoi_simd_parse_until_invalid_signed(s)
    }
}

impl ParserPos<u128> for u128 {
    #[inline(always)]
    fn atoi_simd_parse_pos(s: &[u8]) -> Result<u128, AtoiSimdError> {
        parse_simd_checked_u128(s)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid_pos(s: &[u8]) -> Result<(u128, usize), AtoiSimdError> {
        unsafe { parse_simd_u128(s) }
    }
}
impl Parser<u128> for u128 {}

impl ParserPos<i128> for i128 {
    #[inline(always)]
    fn atoi_simd_parse_pos(s: &[u8]) -> Result<i128, AtoiSimdError> {
        parse_simd_checked_i128(s)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid_pos(s: &[u8]) -> Result<(i128, usize), AtoiSimdError> {
        parse_simd_i128(s)
    }
}

impl ParserNeg<i128> for i128 {
    #[inline(always)]
    fn atoi_simd_parse_neg(s: &[u8]) -> Result<i128, AtoiSimdError> {
        parse_simd_checked_i128_neg(s)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid_neg(s: &[u8]) -> Result<(i128, usize), AtoiSimdError> {
        parse_simd_i128_neg(s)
    }
}

impl Parser<i128> for i128 {
    #[inline(always)]
    fn atoi_simd_parse(s: &[u8]) -> Result<i128, AtoiSimdError> {
        atoi_simd_parse_signed(s)
    }

    #[inline(always)]
    fn atoi_simd_parse_until_invalid(s: &[u8]) -> Result<(i128, usize), AtoiSimdError> {
        atoi_simd_parse_until_invalid_signed(s)
    }
}
