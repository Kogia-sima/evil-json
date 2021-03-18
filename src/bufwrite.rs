use crate::raw::SerializeRaw;

use std::io;

/// A `BufWrite` is a type of `Write`r which has an internal buffer
///
/// This trait is unsafe because incorrect implementation of `next_ptr` or
/// `reserve` method causes UB.
#[doc(hidden)]
pub unsafe trait BufWrite: io::Write {
    fn next_ptr(&mut self) -> *mut u8;
    fn reserve(&mut self, additional: usize) -> Result<(), io::Error>;
    unsafe fn advance(&mut self, additional: usize);
    unsafe fn shrink(&mut self, shrink_size: usize);

    #[inline]
    fn write1<T1: SerializeRaw>(&mut self, value: &T1) -> Result<(), io::Error> {
        let hint = value.size_hint();
        self.reserve(hint).map(|_| unsafe {
            let offset = value.write_to_ptr(self.next_ptr());
            self.advance(offset);
        })
    }

    #[inline]
    fn write2<T1: SerializeRaw, T2: SerializeRaw>(
        &mut self,
        value1: &T1,
        value2: &T2,
    ) -> Result<(), io::Error> {
        let hint = value1.size_hint().saturating_add(value2.size_hint());
        self.reserve(hint).map(|_| unsafe {
            let ptr = self.next_ptr();
            let mut offset = value1.write_to_ptr(ptr);
            offset += value2.write_to_ptr(ptr.add(offset));
            self.advance(offset);
        })
    }

    #[inline]
    fn write3<T1: SerializeRaw, T2: SerializeRaw, T3: SerializeRaw>(
        &mut self,
        value1: &T1,
        value2: &T2,
        value3: &T3,
    ) -> Result<(), io::Error> {
        let hint = value1
            .size_hint()
            .saturating_add(value2.size_hint())
            .saturating_add(value3.size_hint());
        self.reserve(hint).map(|_| unsafe {
            let ptr = self.next_ptr();
            let mut offset = value1.write_to_ptr(ptr);
            offset += value2.write_to_ptr(ptr.add(offset));
            offset += value3.write_to_ptr(ptr.add(offset));
            self.advance(offset);
        })
    }

    #[inline]
    fn write4<T1: SerializeRaw, T2: SerializeRaw, T3: SerializeRaw, T4: SerializeRaw>(
        &mut self,
        value1: &T1,
        value2: &T2,
        value3: &T3,
        value4: &T4,
    ) -> Result<(), io::Error> {
        let hint = value1
            .size_hint()
            .saturating_add(value2.size_hint())
            .saturating_add(value3.size_hint())
            .saturating_add(value4.size_hint());
        self.reserve(hint).map(|_| unsafe {
            let ptr = self.next_ptr();
            let mut offset = value1.write_to_ptr(ptr);
            offset += value2.write_to_ptr(ptr.add(offset));
            offset += value3.write_to_ptr(ptr.add(offset));
            offset += value4.write_to_ptr(ptr.add(offset));
            self.advance(offset);
        })
    }
}

unsafe impl BufWrite for Vec<u8> {
    #[inline]
    fn next_ptr(&mut self) -> *mut u8 {
        unsafe { self.as_mut_ptr().add(self.len()) }
    }

    #[inline]
    fn reserve(&mut self, additional: usize) -> Result<(), io::Error> {
        // SAFETY: this operation won't overflow because slice cannot exceeds isize::MAX bytes.
        // https://doc.rust-lang.org/reference/behavior-considered-undefined.html
        if unlikely!(self.len() + additional > self.capacity()) {
            Vec::reserve(self, additional);
        }

        assume!(self.len() + additional <= self.capacity());
        Ok(())
    }

    #[inline]
    unsafe fn advance(&mut self, additional: usize) {
        self.set_len(self.len() + additional);
    }

    #[inline]
    unsafe fn shrink(&mut self, shrink_size: usize) {
        self.set_len(self.len() - shrink_size);
    }
}
