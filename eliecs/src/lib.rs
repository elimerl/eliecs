mod pool;

use std::{
    fmt::Debug,
    num::{NonZeroU32, NonZeroU64},
};

pub use pool::Pool;

pub use eliecs_macros::components;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Entity {
    pub id: u32,
    pub version: NonZeroU32,
}

impl Entity {
    pub const fn new(id: u32, version: NonZeroU32) -> Self {
        Self { id, version }
    }

    pub const fn from_bits(bits: u64) -> Option<Self> {
        Some(Self {
            version: match NonZeroU32::new((bits >> 32) as u32) {
                Some(g) => g,
                None => return None,
            },
            id: bits as u32,
        })
    }
    pub const fn to_bits(&self) -> NonZeroU64 {
        unsafe { NonZeroU64::new_unchecked((self.version.get() as u64) << 32 | (self.id as u64)) }
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}v{}", self.id, self.version)
    }
}

#[cfg(test)]
mod tests {
    use crate::Pool;

    #[test]
    fn empty() {
        std::hint::black_box(Pool::<u32>::new());
    }

    #[test]
    fn empty_contains_none() {
        let pool = Pool::<u32>::new();
        assert!(!pool.contains(0));
        assert!(!pool.contains(100));
    }

    #[test]
    fn insert_big_id() {
        let mut pool = Pool::<u32>::new();
        assert!(!pool.contains(0));
        pool.insert(0, 1234);
        assert!(pool.contains(0));

        let mut pool = Pool::<u32>::new();
        assert!(!pool.contains(100));
        pool.insert(100, 1234);
        assert!(pool.contains(100));
        assert!(!pool.contains(99));
    }

    #[test]
    fn get() {
        let mut pool = Pool::<u32>::new();
        assert!(!pool.contains(100));
        pool.insert(100, 1234);
        assert!(pool.contains(100));
        assert_eq!(pool.get(100).copied(), Some(1234));
    }

    #[test]
    fn remove() {
        let mut pool = Pool::<u32>::new();
        assert!(!pool.contains(100));
        pool.insert(2, 1234);
        pool.insert(5, 5678);
        pool.insert(7, 91011);
        assert!(pool.contains(2));
        assert_eq!(pool.get(2).copied(), Some(1234));
        assert!(pool.contains(5));
        assert!(pool.contains(7));
        pool.remove(5);
        assert!(pool.contains(2));
        assert!(!pool.contains(5));
        assert!(pool.contains(7));
    }
}
