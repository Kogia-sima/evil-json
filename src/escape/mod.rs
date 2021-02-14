mod char;
mod common;
mod naive;
mod need_escape;

pub(crate) use self::char::escape_char;
pub use need_escape::need_escape;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "avx2",
    not(miri)
))]
mod avx2;

#[cfg(not(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "avx2",
    not(miri)
)))]
pub(crate) use naive::escape;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "avx2"
))]
pub(crate) use avx2::escape;

use crate::bufwrite::BufWrite;

#[cold]
#[inline(never)]
pub(crate) fn escape_cold<W: BufWrite>(buf: &mut W, feed: &str) -> std::io::Result<()> {
    escape(buf, feed)
}
