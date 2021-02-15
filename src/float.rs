pub trait FiniteFloat {
    const MAX_LEN: usize;
    unsafe fn write_to(self, dst: *mut u8) -> usize;
}

macro_rules! finite_float {
    ($name:ident, $type:ty, $max_len: expr, $formatter: ident) => {
        #[derive(Clone, Copy)]
        pub struct $name($type);

        impl $name {
            #[inline]
            pub fn new(val: $type) -> Option<Self> {
                if val.is_finite() {
                    Some(Self(val))
                } else {
                    None
                }
            }
        }

        impl FiniteFloat for $name {
            const MAX_LEN: usize = $max_len;

            #[inline]
            unsafe fn write_to(self, dst: *mut u8) -> usize {
                ryu::raw::$formatter(self.0, dst)
            }
        }
    };
}

finite_float!(FiniteF32, f32, 16, format32);
finite_float!(FiniteF64, f64, 24, format64);
