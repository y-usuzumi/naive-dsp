use std::ops::{Add};
use std::iter::{IntoIterator};
use std::vec;

pub trait Seq<T> : IntoIterator {
}


///////////////////////////////
// FiniteSeq
///////////////////////////////

pub struct FiniteSeq<T> {
    data: Vec<T>
}

impl<T> IntoIterator for FiniteSeq<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Seq<T> for FiniteSeq<T> {
}

impl<T: Add<Output=T>> Add for FiniteSeq<T> {
    type Output = FiniteSeq<T>;

    fn add(self, other: Self) -> Self {
        panic!("Not implemented")
    }
}


///////////////////////////////
// InfiniteSeq
///////////////////////////////

pub enum InfiniteSeq<T> {
    Nil,
    Data(T, Box<InfiniteSeq<T>>)
}

pub struct InfiniteSeqIterator<T> {
    seq: InfiniteSeq<T>
}

impl<T> Iterator for InfiniteSeqIterator<T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.seq {
            InfiniteSeq::Nil => None,
            InfiniteSeq::Data(item, next) => {
                let unboxed_next: InfiniteSeq<_> = *next;
                self.seq = unboxed_next;
                Some(item)
            }
        }
    }
}

impl<T> IntoIterator for InfiniteSeq<T> {
    type Item = T;
    type IntoIter = InfiniteSeqIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        InfiniteSeqIterator { seq: self }
    }
}
