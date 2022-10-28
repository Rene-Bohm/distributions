pub trait Discrete {
    fn lower_bound() -> Self;
    fn upper_bound() -> Self;
}

macro_rules! discrete_impl {
    ($ty:ty, $low:expr, $up:expr) => {
        impl Discrete for $ty {
            fn lower_bound() -> Self {
                $low as $ty
            }
            fn upper_bound() -> Self {
                $up as $ty
            }
        }
    };
}

discrete_impl!(u64, 0, u64::MAX);
discrete_impl!(u32, 0, u32::MAX);
discrete_impl!(u16, 0, u16::MAX);
discrete_impl!(u8, 0, u8::MAX);

discrete_impl!(i64, i64::MIN, i64::MAX);
discrete_impl!(i32, i32::MIN, u32::MAX);
discrete_impl!(i16, i16::MIN, i16::MAX);
discrete_impl!(i8, i8::MIN, i8::MAX);

pub trait Continuous {
    fn lower_bound() -> Self;
    fn upper_bound() -> Self;
}

macro_rules! continuous_impl {
    ($ty:ty, $low:expr, $up:expr) => {
        impl Continuous for $ty {
            fn lower_bound() -> Self {
                $low as $ty
            }
            fn upper_bound() -> Self {
                $up as $ty
            }
        }
    };
}

continuous_impl!(f64, f64::MIN, f64::MAX);
continuous_impl!(f32, f32::MIN, f32::MAX);

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
