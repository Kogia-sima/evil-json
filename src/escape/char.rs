use super::common::{ESCAPED, UU};
use crate::bufwrite::BufWrite;

use std::io;

const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;
const MAX_ONE_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0x10000;

static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";

pub(crate) fn escape_char<W: BufWrite>(buf: &mut W, c: char) -> io::Result<()> {
    tri!(buf.reserve(6));
    let ptr = buf.next_ptr();

    unsafe {
        let len = escape_char_impl(ptr, c);
        buf.advance(len);
    }

    Ok(())
}

unsafe fn escape_char_impl(dst: *mut u8, c: char) -> usize {
    let code = c as u32;
    if code < MAX_ONE_B {
        let escape = ESCAPED[code as usize];
        if likely!(escape == [0, 0]) {
            *dst.add(0) = code as u8;
            1
        } else {
            *dst.add(0) = escape[0];
            *dst.add(1) = escape[1];
            if likely!(escape != UU) {
                2
            } else {
                *dst.add(2) = b'0';
                *dst.add(3) = b'0';
                *dst.add(4) = HEX_DIGITS[(code as u8 >> 4) as usize];
                *dst.add(5) = HEX_DIGITS[(code as u8 & 0xF) as usize];
                6
            }
        }
    } else if code < MAX_TWO_B {
        *dst.add(0) = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
        *dst.add(1) = (code & 0x3F) as u8 | TAG_CONT;
        2
    } else if code < MAX_THREE_B {
        *dst.add(0) = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
        *dst.add(1) = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        *dst.add(2) = (code & 0x3F) as u8 | TAG_CONT;
        3
    } else {
        *dst.add(0) = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
        *dst.add(1) = (code >> 12 & 0x3F) as u8 | TAG_CONT;
        *dst.add(2) = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        *dst.add(3) = (code & 0x3F) as u8 | TAG_CONT;
        4
    }
}
