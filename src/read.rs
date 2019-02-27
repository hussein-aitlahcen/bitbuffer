use crate::{BitStream, Endianness, IsPadded, Result};

/// Trait for types that can be read from a stream without requiring the size to be configured
pub trait Read<'a, E: Endianness, P: IsPadded>: Sized {
    /// Read the type from stream
    fn read(stream: &mut BitStream<'a, E, P>) -> Result<Self>;
}

macro_rules! impl_read_int {
    ($type:ty, $len:expr) => {
        impl<'a, E: Endianness, P: IsPadded> Read<'a, E, P> for $type {
            #[inline(always)]
            fn read(stream: &mut BitStream<'a, E, P>) -> Result<$type> {
                stream.read_int::<$type>($len)
            }
        }
    };
}

impl_read_int!(u8, 8);
impl_read_int!(u16, 16);
impl_read_int!(u32, 32);
impl_read_int!(u64, 64);
impl_read_int!(u128, 128);
impl_read_int!(i8, 8);
impl_read_int!(i16, 16);
impl_read_int!(i32, 32);
impl_read_int!(i64, 64);
impl_read_int!(i128, 128);

impl<'a, E: Endianness, P: IsPadded> Read<'a, E, P> for f32 {
    #[inline(always)]
    fn read(stream: &mut BitStream<'a, E, P>) -> Result<f32> {
        stream.read_float::<f32>()
    }
}

impl<'a, E: Endianness, P: IsPadded> Read<'a, E, P> for f64 {
    #[inline(always)]
    fn read(stream: &mut BitStream<'a, E, P>) -> Result<f64> {
        stream.read_float::<f64>()
    }
}

impl<'a, E: Endianness, P: IsPadded> Read<'a, E, P> for bool {
    #[inline(always)]
    fn read(stream: &mut BitStream<'a, E, P>) -> Result<bool> {
        stream.read_bool()
    }
}

impl<'a, E: Endianness, P: IsPadded> Read<'a, E, P> for String {
    #[inline(always)]
    fn read(stream: &mut BitStream<'a, E, P>) -> Result<String> {
        stream.read_string(None)
    }
}

/// Trait for types that can be read from a stream wit requiring the size to be configured
pub trait ReadSize<'a, E: Endianness, P: IsPadded>: Sized {
    /// Read the type from stream
    fn read(stream: &mut BitStream<'a, E, P>, size: usize) -> Result<Self>;
}

macro_rules! impl_read_int_sized {
    ($type:ty) => {
        impl<'a, E: Endianness, P: IsPadded> ReadSize<'a, E, P> for $type {
            #[inline(always)]
            fn read(stream: &mut BitStream<'a, E, P>, size: usize) -> Result<$type> {
                stream.read_int::<$type>(size)
            }
        }
    };
}

impl_read_int_sized!(u8);
impl_read_int_sized!(u16);
impl_read_int_sized!(u32);
impl_read_int_sized!(u64);
impl_read_int_sized!(u128);
impl_read_int_sized!(i8);
impl_read_int_sized!(i16);
impl_read_int_sized!(i32);
impl_read_int_sized!(i64);
impl_read_int_sized!(i128);

impl<'a, E: Endianness, P: IsPadded> ReadSize<'a, E, P> for String {
    #[inline(always)]
    fn read(stream: &mut BitStream<'a, E, P>, size: usize) -> Result<String> {
        stream.read_string(Some(size))
    }
}
