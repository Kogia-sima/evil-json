const TR: bool = true;
const __: bool = false;

#[rustfmt::skip]
const ESCAPE: [bool; 256] = [
    TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,
    TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,TR,
    __,__,TR,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,TR,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
    __,__,__,__,__,__,__,__,__,__,__,__,__,__,__,__,
];

// This function will be evaluated at compilation time under the following environment.
//
// * `feed.len() < 100`
// * rustc version >= 1.38.0 (LLVM >= 9)
// * opt-level is at least 2
//
// compile-time evaluation also works on rustc < 1.38.0, but `feed` must be shorter than
// 30 bytes.
#[cfg_attr(not(debug_assertions), inline(always))]
pub fn need_escape(feed: &'static str) -> bool {
    let bytes = feed.as_bytes();

    let mut i = 0;
    while i < feed.len() {
        if ESCAPE[bytes[i] as usize] {
            return true;
        }

        i += 1;
    }

    false
}
