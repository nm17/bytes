use std::iter::{Copied, Enumerate};
use nom::{Compare, CompareResult, FindSubstring, InputLength, InputTake, InputTakeAtPosition, IResult, Needed, Slice, UnspecializedInput};
use nom::error::{ErrorKind, ParseError};
use crate::Bytes;

impl InputLength for Bytes {
    fn input_len(&self) -> usize {
        dbg!(&self, &self.len(), self.as_ref().len());
        self.as_ref().len()
    }
}

use alloc::boxed::Box;
use core::slice::{Iter};
use std::cmp::min;
use std::dbg;
use std::ops::RangeFrom;


impl nom::InputIter for Bytes {
    type Item = u8;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = Box<dyn Iterator<Item = u8>>;

    fn iter_indices(&self) -> Self::Iter {
        self.iter_elements().enumerate()
    }

    fn iter_elements(&self) -> Self::IterElem {
        Box::new(self.clone().into_iter())
    }

    fn position<P>(&self, predicate: P) -> Option<usize> where P: Fn(Self::Item) -> bool {
        self.iter().copied().position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.len() >= count {
            Ok(count)
        } else {
            Err(Needed::new(count - self.len()))
        }
    }
}

impl InputTake for Bytes {
    fn take(&self, count: usize) -> Self {
        assert!(count <= self.len());
        self.slice(0..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        assert!(count <= self.len());
        (
            self.slice(count..self.len()),
            self.slice(0..count),
        )
    }
}

impl Slice<RangeFrom<usize>> for Bytes {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range)
    }
}

/*impl Compare<Bytes> for Bytes {
    fn compare(&self, t: Bytes) -> CompareResult {
        return self.as_ref().compare(t.as_ref())
    }

    fn compare_no_case(&self, t: Bytes) -> CompareResult {
        return self.as_ref().compare_no_case(t.as_ref())
    }
}*/

impl FindSubstring<Bytes> for Bytes {
    fn find_substring(&self, substr: Bytes) -> Option<usize> {
        self.as_ref().find_substring(substr.as_ref())
    }
}



impl UnspecializedInput for Bytes {}
