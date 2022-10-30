pub trait Discrete {
    fn lower_bound() -> Self;
    fn upper_bound() -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! discrete_impl {
    ($ty:ty, $low:expr, $up:expr, $zero:expr, $one:expr) => {
        impl Discrete for $ty {
            fn lower_bound() -> Self {
                $low as $ty
            }
            fn upper_bound() -> Self {
                $up as $ty
            }
            fn zero() -> Self{
                $zero as $ty
            }
            fn one() -> Self{
                $one as $ty
            }
        }
    };
}

discrete_impl!(u64, 0, u64::MAX, 0, 1);
discrete_impl!(u32, 0, u32::MAX, 0, 1);
discrete_impl!(u16, 0, u16::MAX, 0, 1);
discrete_impl!(u8, 0, u8::MAX, 0, 1);

discrete_impl!(i64, i64::MIN, i64::MAX, 0, 1);
discrete_impl!(i32, i32::MIN, u32::MAX, 0, 1);
discrete_impl!(i16, i16::MIN, i16::MAX, 0, 1);
discrete_impl!(i8, i8::MIN, i8::MAX, 0, 1);

pub trait Continuous {
    fn lower_bound() -> Self;
    fn upper_bound() -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! continuous_impl {
    ($ty:ty, $low:expr, $up:expr, $zero:expr, $one:expr) => {
        impl Continuous for $ty {
            fn lower_bound() -> Self {
                $low as $ty
            }
            fn upper_bound() -> Self {
                $up as $ty
            }
            fn zero() -> Self{
                $zero as $ty
            }
            fn one() -> Self{
                $one as $ty
            }
        }
    };
}

continuous_impl!(f64, f64::MIN, f64::MAX, 0.0, 1.0);
continuous_impl!(f32, f32::MIN, f32::MAX, 0.0, 1.0);

pub trait Signed {}

impl Signed for i64 {}
impl Signed for i32 {}
impl Signed for i16 {}
impl Signed for i8 {}

impl Signed for f64 {}
impl Signed for f32 {}

pub trait Unsigned {}

impl Unsigned for u64 {}
impl Unsigned for u32 {}
impl Unsigned for u16 {}
impl Unsigned for u8 {}
