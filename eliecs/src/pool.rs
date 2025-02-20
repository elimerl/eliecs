use std::marker::PhantomData;

use serde::{de::Visitor, ser::SerializeTupleStruct};

type Index = u32;

#[derive(Clone, Debug)]
pub struct Pool<T> {
    sparse: Vec<Index>,
    dense: Vec<(Index, T)>,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
        }
    }

    pub fn with_dense_capacity(cap: u32) -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::with_capacity(cap as usize),
        }
    }

    pub fn contains(&self, i: Index) -> bool {
        (i as usize) < self.sparse.len() && {
            let dense_idx = self.sparse[i as usize] as usize;
            dense_idx < self.dense.len() && self.dense[dense_idx].0 == i
        }
    }

    pub fn insert(&mut self, i: Index, v: T) -> bool {
        let already_existed = self.contains(i);
        let dense_idx = self.dense.len() as Index;
        if self.sparse.len() < (i as usize + 1) {
            // fills new empty space with u32::MAX
            self.sparse.resize(i as usize + 1, u32::MAX);
        }
        self.sparse[i as usize] = dense_idx;
        self.dense.push((i, v));
        already_existed
    }

    pub fn get(&self, i: Index) -> Option<&T> {
        if !self.contains(i) {
            return None;
        }
        let dense_idx = self.sparse[i as usize] as usize;
        if dense_idx < self.dense.len() {
            Some(&self.dense[dense_idx].1)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, i: Index) -> Option<&mut T> {
        if !self.contains(i) {
            return None;
        }
        let dense_idx = self.sparse[i as usize] as usize;
        if dense_idx < self.dense.len() {
            Some(&mut self.dense[dense_idx].1)
        } else {
            None
        }
    }

    pub fn remove(&mut self, i: Index) -> bool {
        if !self.contains(i) {
            return false;
        }

        let tail_dense = self.dense.pop().unwrap();
        let tail_sparse_ref = tail_dense.0;
        if (self.sparse[i as usize] as usize) < self.dense.len() {
            self.dense[self.sparse[i as usize] as usize] = tail_dense;
        }
        self.sparse[tail_sparse_ref as usize] = self.sparse[i as usize];

        true
    }

    pub fn clear(&mut self) {
        self.sparse.clear();
        self.dense.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &T)> {
        self.dense.iter().map(|v| (v.0, &v.1))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut T)> {
        self.dense.iter_mut().map(|v| (v.0, &mut v.1))
    }

    pub fn len(&self) -> u32 {
        self.dense.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }
}

impl<T> serde::Serialize for Pool<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("Pool", 2)?;
        s.serialize_field(&self.sparse)?;
        s.serialize_field(&self.dense)?;
        s.end()
    }
}

impl<'de, T> serde::Deserialize<'de> for Pool<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PoolVisitor<T>(PhantomData<T>);
        impl<'de, T> Visitor<'de> for PoolVisitor<T>
        where
            T: serde::Deserialize<'de>,
        {
            type Value = Pool<T>;
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let sparse = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let dense = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                Ok(Pool { sparse, dense })
            }

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a serialized Pool")
            }
        }
        deserializer.deserialize_tuple(2, PoolVisitor::<T>(PhantomData))
    }
}

impl<T> Default for Pool<T> {
    fn default() -> Self {
        Self::new()
    }
}
