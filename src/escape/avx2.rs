use super::common::{u_encode, BS, ESCAPED, QU, UU};
use crate::bufwrite::BufWrite;

use std::io;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline]
unsafe fn memcpy_16_32(src: *const u8, dst: *mut u8, len: usize) {
    debug_assert!(len >= 16);
    debug_assert!(len <= 32);

    let offset = len - 16;
    let v1 = _mm_loadu_si128(src as _);
    let v2 = _mm_loadu_si128(src.add(offset) as _);
    _mm_storeu_si128(dst as _, v1);
    _mm_storeu_si128(dst.add(offset) as _, v2);
}

#[inline]
unsafe fn escape_middle<W: BufWrite>(buf: &mut W, feed: &str) -> io::Result<()> {
    debug_assert!(feed.len() >= 16);
    debug_assert!(feed.len() <= 32);

    let mut tmp = [0xff; 32];
    memcpy_16_32(feed.as_ptr(), tmp.as_mut_ptr(), feed.len());

    let v_offset = _mm256_set1_epi8(-128);
    let v_bound = _mm256_set1_epi8(-96);
    let v_quot = _mm256_set1_epi8(QU[1] as i8);
    let v_bs = _mm256_set1_epi8(BS[1] as i8);

    let v = _mm256_loadu_si256(tmp.as_ptr() as _);

    let mut mask = _mm256_movemask_epi8(_mm256_or_si256(
        _mm256_cmpgt_epi8(v_bound, _mm256_add_epi8(v, v_offset)),
        _mm256_or_si256(_mm256_cmpeq_epi8(v, v_quot), _mm256_cmpeq_epi8(v, v_bs)),
    )) as u32;

    if mask == 0 {
        tri!(buf.reserve(32));
        _mm256_storeu_si256(buf.next_ptr() as _, v);
        buf.advance(feed.len());
        return Ok(());
    }

    let ptr = tmp.as_ptr();
    let mut read_ptr = ptr;
    let end_ptr = tmp[feed.len()..].as_ptr();

    while mask != 0 {
        let trailing_zeros = mask.trailing_zeros() as usize;
        mask ^= 1 << trailing_zeros;
        let ptr2 = ptr.add(trailing_zeros);
        if read_ptr < ptr2 {
            let part =
                std::slice::from_raw_parts(read_ptr, ptr2 as usize - read_ptr as usize);
            tri!(buf.write_all(part));
        }

        read_ptr = ptr2.add(1);
        let key = ESCAPED[*ptr2 as usize];
        if key != UU {
            tri!(buf.write_all(&key[..]));
        } else {
            tri!(u_encode(buf, *ptr2));
        }
    }

    if read_ptr != end_ptr {
        let part =
            std::slice::from_raw_parts(read_ptr, end_ptr as usize - read_ptr as usize);
        buf.write_all(part)
    } else {
        Ok(())
    }
}

#[inline]
unsafe fn escape_avx2<B: BufWrite>(buf: &mut B, feed: &str) -> io::Result<()> {
    debug_assert!(feed.len() >= 32);

    let mut ptr = feed.as_ptr();
    let mut read_ptr = ptr;
    let end_ptr = feed[feed.len()..].as_ptr();

    let v_bound = _mm256_set1_epi8(0x1f);
    let v_quot = _mm256_set1_epi8(QU[1] as i8);
    let v_bs = _mm256_set1_epi8(BS[1] as i8);

    while end_ptr as usize - ptr as usize >= 32 {
        let v = _mm256_loadu_si256(ptr as _);
        let mut mask = _mm256_movemask_epi8(_mm256_or_si256(
            _mm256_cmpeq_epi8(_mm256_min_epu8(v, v_bound), v),
            _mm256_or_si256(_mm256_cmpeq_epi8(v, v_quot), _mm256_cmpeq_epi8(v, v_bs)),
        )) as u32;

        while mask != 0 {
            let trailing_zeros = mask.trailing_zeros() as usize;
            mask ^= 1 << trailing_zeros;
            let ptr2 = ptr.add(trailing_zeros);
            if read_ptr < ptr2 {
                let part = std::slice::from_raw_parts(
                    read_ptr,
                    ptr2 as usize - read_ptr as usize,
                );
                tri!(buf.write_all(part));
            }

            read_ptr = ptr2.add(1);
            let key = ESCAPED[*ptr2 as usize];
            if key != UU {
                tri!(buf.write_all(&key[..]));
            } else {
                tri!(u_encode(buf, *ptr2));
            }
        }

        ptr = ptr.add(32);
    }

    if ptr != end_ptr {
        let v = _mm256_loadu_si256(end_ptr.sub(32) as _);
        let mut mask = _mm256_movemask_epi8(_mm256_or_si256(
            _mm256_cmpeq_epi8(_mm256_min_epu8(v, v_bound), v),
            _mm256_or_si256(_mm256_cmpeq_epi8(v, v_quot), _mm256_cmpeq_epi8(v, v_bs)),
        )) as u32;
        mask >>= ptr as usize - end_ptr.sub(32) as usize;

        while mask != 0 {
            let trailing_zeros = mask.trailing_zeros() as usize;
            mask ^= 1 << trailing_zeros;
            let ptr2 = ptr.add(trailing_zeros);
            if read_ptr < ptr2 {
                let part = std::slice::from_raw_parts(
                    read_ptr,
                    ptr2 as usize - read_ptr as usize,
                );
                tri!(buf.write_all(part));
            }

            read_ptr = ptr2.add(1);
            let key = ESCAPED[*ptr2 as usize];
            if key != UU {
                tri!(buf.write_all(&key[..]));
            } else {
                tri!(u_encode(buf, *ptr2));
            }
        }
    }

    if read_ptr < end_ptr {
        let part =
            std::slice::from_raw_parts(read_ptr, end_ptr as usize - read_ptr as usize);
        buf.write_all(part)
    } else {
        Ok(())
    }
}

pub(crate) fn escape<B: BufWrite>(buf: &mut B, feed: &str) -> io::Result<()> {
    if feed.len() <= 16 {
        super::naive::escape(buf, feed)
    } else if feed.len() <= 32 {
        unsafe { escape_middle(buf, feed) }
    } else {
        unsafe { escape_avx2(buf, feed) }
    }
}
