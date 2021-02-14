use crate::float::{FiniteF32, FiniteF64, FiniteFloat};

use itoap::Integer;
use std::ptr::copy_nonoverlapping;

#[doc(hidden)]
pub trait SerializeRaw {
    fn size_hint(&self) -> usize;
    unsafe fn write_to_ptr(&self, dst: *mut u8) -> usize;
}

// String which does not need escape
pub struct RawStr(pub &'static str);

impl SerializeRaw for RawStr {
    #[inline]
    fn size_hint(&self) -> usize {
        self.0.len()
    }

    #[inline]
    unsafe fn write_to_ptr(&self, dst: *mut u8) -> usize {
        copy_nonoverlapping(self.0.as_ptr(), dst, self.0.len());
        self.0.len()
    }
}

impl SerializeRaw for bool {
    #[inline]
    fn size_hint(&self) -> usize {
        5
    }

    #[inline]
    unsafe fn write_to_ptr(&self, dst: *mut u8) -> usize {
        if *self {
            copy_nonoverlapping(b"true".as_ptr(), dst, 4);
            4
        } else {
            copy_nonoverlapping(b"false".as_ptr(), dst, 5);
            5
        }
    }
}

macro_rules! impl_integers {
    ($($type:ty),*) => {
        $(
            impl SerializeRaw for $type {
                #[inline]
                fn size_hint(&self) -> usize {
                    <$type as Integer>::MAX_LEN
                }

                #[inline]
                unsafe fn write_to_ptr(&self, dst: *mut u8) -> usize {
                    <$type as Integer>::write_to(*self, dst)
                }
            }
        )*
    };
}

impl_integers!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_floating_points {
    ($($type:ty),*) => {
        $(
            impl SerializeRaw for $type {
                #[inline]
                fn size_hint(&self) -> usize {
                    <$type as FiniteFloat>::MAX_LEN
                }

                #[inline]
                unsafe fn write_to_ptr(&self, dst: *mut u8) -> usize {
                    <$type as FiniteFloat>::write_to(*self, dst)
                }
            }
        )*
    };
}

impl_floating_points!(FiniteF32, FiniteF64);
