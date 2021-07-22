use core::cmp::{max, min};

use alloc::vec::Vec;

pub struct IdTable<T> {
    inner: Vec<Option<T>>,
}

impl<T> IdTable<T> {
    pub fn new() -> IdTable<T> {
        IdTable { inner: Vec::new() }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if index >= self.inner.len() {
            self.inner.resize_with(index + 1, || None);
        }

        self.inner[index] = Some(value);
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.inner.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match self.inner.get(index) {
            Some(value) => value.as_ref(),
            None => None,
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match self.inner.get_mut(index) {
            Some(value) => value.as_mut(),
            None => None,
        }
    }

    pub fn get_mut2(&mut self, index1: usize, index2: usize) -> (Option<&mut T>, Option<&mut T>) {
        match (index1 < self.inner.len(), index2 < self.inner.len()) {
            (true, true) => {
                let low = min(index1, index2);
                let high = max(index1, index2);
                let (left, right) = self.inner.as_mut_slice().split_at_mut(high);
                (left[low].as_mut(), right[0].as_mut())
            }
            (true, false) => (self.inner[index1].as_mut(), None),
            (false, true) => (None, self.inner[index2].as_mut()),
            (false, false) => (None, None),
        }
    }
}
