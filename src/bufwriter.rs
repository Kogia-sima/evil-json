use crate::bufwrite::BufWrite;

use core::cmp;
use core::ptr;
use std::io;
use std::io::Write;

// We allocate memory slightly larger than `std::io::Bufwriter` so that
// `std::io::BufWriter` wrapped by evil_json's `BufWriter` does not copy buffer
// contents every time
const DEFAULT_BUFFER_SIZE: usize = 8192 + 256;
const MIN_BUFFER_SIZE: usize = 48;

pub struct BufWriter<'a> {
    inner: Box<dyn 'a + Write>,
    buf: Vec<u8>,
}

impl<'a> BufWriter<'a> {
    #[inline]
    pub fn new<W: 'a + Write>(inner: W) -> BufWriter<'a> {
        Self::with_capacity(DEFAULT_BUFFER_SIZE, inner)
    }

    #[inline]
    pub fn with_capacity<W: 'a + Write>(mut capacity: usize, inner: W) -> BufWriter<'a> {
        capacity = cmp::max(MIN_BUFFER_SIZE, capacity);

        BufWriter {
            inner: Box::new(inner),
            buf: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    unsafe fn write_to_buf_unchecked(&mut self, data: &[u8]) {
        let old_len = self.buf.len();
        let buf_len = data.len();
        let src = data.as_ptr();
        let dst = self.buf.as_mut_ptr().add(old_len);
        ptr::copy_nonoverlapping(src, dst, buf_len);
        self.buf.set_len(old_len + buf_len);
    }

    #[cold]
    #[inline(never)]
    fn write_slow(&mut self, data: &[u8]) -> io::Result<()> {
        tri!(self.flush_buf());

        if data.len() <= self.buf.capacity() {
            unsafe {
                self.write_to_buf_unchecked(data);
            }
            Ok(())
        } else {
            self.inner.write_all(data)
        }
    }

    pub(crate) fn flush_buf(&mut self) -> io::Result<()> {
        tri!(self.inner.write_all(self.buf.as_slice()));
        self.buf.clear();
        Ok(())
    }
}

impl<'a> Write for BufWriter<'a> {
    #[inline]
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.write_all(data).map(|_| data.len())
    }

    #[inline]
    fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
        if self.buf.len() + data.len() <= self.buf.capacity() {
            unsafe {
                self.write_to_buf_unchecked(data);
            }
            Ok(())
        } else {
            self.write_slow(data)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush_buf().and_then(|()| self.inner.flush())
    }
}

unsafe impl<'a> BufWrite for BufWriter<'a> {
    #[inline]
    fn next_ptr(&mut self) -> *mut u8 {
        self.buf.next_ptr()
    }

    #[inline]
    fn reserve(&mut self, additional: usize) -> Result<(), io::Error> {
        // SAFETY: this operation won't overflow because slice cannot exceeds isize::MAX bytes.
        // https://doc.rust-lang.org/reference/behavior-considered-undefined.html
        if likely!(self.buf.len() + additional <= self.buf.capacity()) {
            Ok(())
        } else if likely!(additional <= self.buf.capacity()) {
            self.flush_buf()
        } else {
            // this case should never happen in this crate. all input data
            // must be shorter than MIN_BUFFER_SIZE.
            Err(io::Error::new(io::ErrorKind::Other, "capacity overflow"))
        }
    }

    #[inline]
    unsafe fn advance(&mut self, additional: usize) {
        self.buf.advance(additional);
    }

    #[inline]
    unsafe fn shrink(&mut self, shrink_size: usize) {
        self.buf.shrink(shrink_size);
    }
}

impl<'a> Drop for BufWriter<'a> {
    fn drop(&mut self) {
        let _ = self.flush_buf();
    }
}

#[cfg(test)]
mod tests {
    use super::{BufWriter, MIN_BUFFER_SIZE};
    use crate::bufwrite::BufWrite;
    use std::io::Write;

    #[test]
    fn write() {
        let inner = Vec::new();
        let mut writer = BufWriter::with_capacity(2, inner);
        assert_eq!(writer.buf, &[]);
        assert_eq!(writer.buf.capacity(), MIN_BUFFER_SIZE);

        writer.write(&[]).unwrap();
        assert_eq!(writer.buf, &[]);
        assert_eq!(writer.buf.capacity(), MIN_BUFFER_SIZE);

        writer.write(b"0000").unwrap();
        assert_eq!(writer.buf, b"0000");
        assert_eq!(writer.buf.capacity(), MIN_BUFFER_SIZE);

        let remain = writer.buf.capacity() - writer.buf.len();
        writer.write(b"1".repeat(remain).as_slice()).unwrap();
        assert_eq!(writer.buf.len(), MIN_BUFFER_SIZE);

        writer.write(b"2").unwrap();
        assert_eq!(writer.buf, b"2");
        assert_eq!(writer.buf.capacity(), MIN_BUFFER_SIZE);

        writer.flush().unwrap();
        assert_eq!(writer.buf, b"");
        assert_eq!(writer.buf.capacity(), MIN_BUFFER_SIZE);
    }

    #[test]
    fn reserve() {
        let inner = Vec::new();
        let mut writer = BufWriter::with_capacity(MIN_BUFFER_SIZE + 1, inner);

        assert_eq!(writer.reserve(MIN_BUFFER_SIZE + 1).ok(), Some(()));
        assert_eq!(writer.reserve(MIN_BUFFER_SIZE + 2).ok(), None);

        writer.write(b"@").unwrap();
        assert_eq!(writer.reserve(MIN_BUFFER_SIZE + 1).ok(), Some(()));
        assert_eq!(writer.buf, b"");
    }
}