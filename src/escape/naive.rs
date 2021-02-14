use super::common::{u_encode, ESCAPED, UU};
use crate::bufwrite::BufWrite;

use std::io;

#[inline]
pub(crate) fn escape<B: BufWrite>(buf: &mut B, feed: &str) -> io::Result<()> {
    let bytes = feed.as_bytes();

    let mut start = 0;
    let mut i = 0;

    while i < bytes.len() {
        let escape = ESCAPED[bytes[i] as usize];
        if escape == [0, 0] {
            i += 1;
        } else {
            if i > start {
                tri!(buf.write_all(&bytes[start..i]));
            }

            if escape != UU {
                tri!(buf.write_all(&escape));
            } else {
                tri!(u_encode(buf, bytes[i]));
            }

            i += 1;
            start = i;
        }
    }

    if start < bytes.len() {
        buf.write_all(&bytes[start..])
    } else {
        Ok(())
    }
}
