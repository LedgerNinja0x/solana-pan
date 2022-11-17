use crate::errors::FankorResult;
use crate::models::{ZeroCopyType, ZC};
use crate::prelude::{FnkSet, FnkUInt};
use borsh::BorshDeserialize;

impl<T: ZeroCopyType> ZeroCopyType for FnkSet<T> {
    fn byte_size_from_instance(&self) -> usize {
        let mut size = 0;

        let len = FnkUInt::from(self.len() as u64);
        size += len.byte_size_from_instance();

        for v in &self.0 {
            size += v.byte_size_from_instance();
        }

        size
    }

    fn byte_size(mut bytes: &[u8]) -> FankorResult<usize> {
        let mut size = 0;

        let bytes2 = &mut bytes;
        let len = FnkUInt::deserialize(bytes2)?;
        size += len.0 as usize;

        for _ in 0..len.0 {
            bytes = &bytes[size..];
            size += T::byte_size(bytes)?;
        }

        Ok(size)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

impl<'info, T: ZeroCopyType> ZC<'info, FnkSet<T>> {
    // GETTERS ----------------------------------------------------------------

    /// The length of the vector.
    pub fn len(&self) -> FankorResult<usize> {
        let bytes = (*self.data).borrow();
        let mut bytes = &bytes[self.offset..];
        let len = FnkUInt::deserialize(&mut bytes)?;

        Ok(len.0 as usize)
    }

    /// Whether the vector is empty or not
    pub fn is_empty(&self) -> FankorResult<bool> {
        Ok(self.len()? == 0)
    }
}

impl<'info, T: ZeroCopyType> IntoIterator for ZC<'info, FnkSet<T>> {
    type Item = ZC<'info, T>;
    type IntoIter = Iter<'info, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            offset: self.offset,
            len: self
                .len()
                .expect("Failed to get length of zc-vector in iterator"),
            data: self,
            index: 0,
        }
    }
}

impl<'a, 'info, T: ZeroCopyType> IntoIterator for &'a ZC<'info, FnkSet<T>> {
    type Item = ZC<'info, T>;
    type IntoIter = Iter<'info, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            offset: self.offset,
            data: self.clone(),
            len: self
                .len()
                .expect("Failed to get length of zc-vector in iterator"),
            index: 0,
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct Iter<'info, T: ZeroCopyType> {
    data: ZC<'info, FnkSet<T>>,
    len: usize,
    index: usize,
    offset: usize,
}

impl<'info, T: ZeroCopyType> Iterator for Iter<'info, T> {
    type Item = ZC<'info, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }

        let result = ZC {
            data: self.data.data.clone(),
            offset: self.offset,
            _phantom: std::marker::PhantomData,
        };

        let bytes = (*self.data.data).borrow();
        let bytes = &bytes[self.offset..];

        self.offset += T::byte_size(bytes).expect("Deserialization failed in set iterator");
        self.index += 1;

        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.len - self.index;

        (size, Some(size))
    }
}

impl<'info, T: ZeroCopyType> ExactSizeIterator for Iter<'info, T> {}