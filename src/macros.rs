macro_rules! tri {
    ($expr:expr) => {
        match $expr {
            ::core::result::Result::Ok(o) => o,
            ::core::result::Result::Err(e) => return ::core::result::Result::Err(e),
        }
    };
}

#[cfg(evil_json_nightly)]
macro_rules! likely {
    ($val:expr) => {
        ::core::intrinsics::likely($val)
    };
}

#[cfg(not(evil_json_nightly))]
macro_rules! likely {
    ($val:expr) => {
        $val
    };
}

#[cfg(evil_json_nightly)]
macro_rules! unlikely {
    ($val:expr) => {
        ::core::intrinsics::unlikely($val)
    };
}

#[cfg(not(evil_json_nightly))]
macro_rules! unlikely {
    ($val:expr) => {
        $val
    };
}

#[cfg(evil_json_nightly)]
macro_rules! assume {
    ($val:expr) => {
        ::core::intrinsics::assume($val)
    };
}

#[cfg(not(evil_json_nightly))]
macro_rules! assume {
    ($val:expr) => {
        debug_assert!($val)
    };
}
