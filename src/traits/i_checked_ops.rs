// suppress `use_self` recommendation; not avoidable in macro context
#![allow(clippy::use_self)]

#[cfg(test)]
mod unit_tests;

pub trait ICheckedOps<T = Self>: Sized {
    #[must_use]
    fn checked_abs(self) -> Option<Self>;
    #[must_use]
    fn checked_add(self, rhs: T) -> Option<Self>;
    #[must_use]
    fn checked_div(self, rhs: T) -> Option<Self>;
    #[must_use]
    fn checked_div_euclid(self, rhs: T) -> Option<Self>;
    #[must_use]
    fn checked_mul(self, rhs: T) -> Option<Self>;
    #[must_use]
    fn checked_neg(self) -> Option<Self>;
    #[must_use]
    fn checked_pow(self, rhs: u32) -> Option<Self>;
    #[must_use]
    fn checked_rem(self, rhs: T) -> Option<Self>;
    #[must_use]
    fn checked_rem_euclid(self, rhs: T) -> Option<Self>;
    #[must_use]
    fn checked_shl(self, rhs: u32) -> Option<Self>;
    #[must_use]
    fn checked_shr(self, rhs: u32) -> Option<Self>;
    #[must_use]
    fn checked_sub(self, rhs: T) -> Option<Self>;
}

macro_rules! checked_impl {
    ($tr:ty, $ret:ty; $($typ:ty),+ $(,)?) => ($(
        impl ICheckedOps for $typ {
            binary_op_impl! {
                $tr, $typ, $ret;
                checked_add,
                checked_div,
                checked_div_euclid,
                checked_mul,
                checked_rem,
                checked_rem_euclid,
                checked_sub
            }

            binary_op_impl! {
                $tr, u32, $ret;
                checked_pow,
                checked_shl,
                checked_shr
            }

            unary_op_impl! {
                $ret;
                checked_abs,
                checked_neg
            }
        }
    )*)
}

checked_impl! { ICheckedOps, Option<Self>; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, }
