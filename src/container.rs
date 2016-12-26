use std::fmt::{ Debug, Formatter, Result };

use util::ExtInt;
use store;
use store::Store;

#[derive(PartialEq, Clone)]
pub struct Container<Size: ExtInt> {
    pub key: Size,
    pub len: u64,
    pub store: Store<Size>,
}

pub struct Iter<'a, Size: ExtInt + 'a> {
    pub key: Size,
    inner: store::Iter<'a, Size>,
}

impl<Size: ExtInt> Container<Size> {
    pub fn new(key: Size) -> Container<Size> {
        Container {
            key: key,
            len: 0,
            store: Store::new(),
        }
    }
}

impl<Size: ExtInt> Container<Size> {
    #[inline]
    pub fn key(&self) -> Size { self.key }

    #[inline]
    pub fn len(&self) -> u64 { self.len }

    #[inline]
    pub fn insert(&mut self, index: Size) -> bool {
        if self.store.insert(index) {
            self.len += 1;
            self.ensure_correct_store();
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn remove(&mut self, index: Size) -> bool {
        if self.store.remove(index) {
            self.len -= 1;
            self.ensure_correct_store();
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn contains(&self, index: Size) -> bool {
        self.store.contains(index)
    }

    #[allow(needless_lifetimes)] // TODO: https://github.com/Manishearth/rust-clippy/issues/740
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<Size> {
        Iter {
            key: self.key,
            inner: self.store.iter()
        }
    }

    #[inline]
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.store.is_disjoint(&other.store)
    }

    #[inline]
    pub fn is_subset(&self, other: &Self) -> bool {
        if self.len > other.len {
            false
        } else {
            self.store.is_subset(&other.store)
        }
    }

    #[inline]
    pub fn union_with(&mut self, other: &Self) {
        self.store.union_with(&other.store);
        self.len = self.store.len();
        self.ensure_correct_store();
    }

    #[inline]
    pub fn intersect_with(&mut self, other: &Self) {
        self.store.intersect_with(&other.store);
        self.len = self.store.len();
        self.ensure_correct_store();
    }

    #[inline]
    pub fn difference_with(&mut self, other: &Self) {
        self.store.difference_with(&other.store);
        self.len = self.store.len();
        self.ensure_correct_store();
    }

    #[inline]
    pub fn symmetric_difference_with(&mut self, other: &Self) {
        self.store.symmetric_difference_with(&other.store);
        self.len = self.store.len();
        self.ensure_correct_store();
    }

    #[inline]
    pub fn min(&self) -> Size {
        self.store.min()
    }

    #[inline]
    pub fn max(&self) -> Size {
        self.store.max()
    }

    pub fn run_optimize(&mut self) -> bool {
        self.store.run_optimize()
    }

    #[inline]
    fn ensure_correct_store(&mut self) {
        self.store.to_array_or_bitmap(Some(self.len));
    }
}

impl<'a, Size: ExtInt> Iterator for Iter<'a, Size> {
    type Item = Size;
    fn next(&mut self) -> Option<Size> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<Size: ExtInt + Debug> Debug for Container<Size> {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        format!("Container<{:?} @ {:?}>", self.len(), self.key()).fmt(formatter)
    }
}
