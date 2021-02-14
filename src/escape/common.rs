use crate::bufwrite::BufWrite;
use std::io;

pub(super) const QU: [u8; 2] = *b"\\\"";
pub(super) const BS: [u8; 2] = *b"\\\\";
pub(super) const BB: [u8; 2] = *b"\\b";
pub(super) const TT: [u8; 2] = *b"\\t";
pub(super) const NN: [u8; 2] = *b"\\n";
pub(super) const FF: [u8; 2] = *b"\\f";
pub(super) const RR: [u8; 2] = *b"\\r";
pub(super) const UU: [u8; 2] = *b"\\u";
const __: [u8; 2] = [0, 0];

#[rustfmt::skip]
pub(super) static ESCAPED: [[u8; 2]; 256] = [
    UU, UU, UU, UU, UU, UU, UU, UU, BB, TT, NN, UU, FF, RR, UU, UU,
    UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU,
    __, __, QU, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, BS, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,
];

#[cold]
pub(super) fn u_encode<B: BufWrite>(buf: &mut B, byte: u8) -> io::Result<()> {
    static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
    let bytes = [
        b'\\',
        b'u',
        b'0',
        b'0',
        HEX_DIGITS[(byte >> 4) as usize],
        HEX_DIGITS[(byte & 0xF) as usize],
    ];
    buf.write_all(&bytes)
}
