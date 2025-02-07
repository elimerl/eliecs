type Index = u32;

#[derive(Clone, Debug)]
pub struct Pool<T> {
    pub sparse: Vec<Index>,
    pub dense_indices: Vec<Index>,
    pub dense_data: Vec<T>,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense_indices: Vec::new(),
            dense_data: Vec::new(),
        }
    }

    pub fn with_dense_capacity(cap: u32) -> Self {
        Self {
            sparse: Vec::new(),
            dense_indices: Vec::with_capacity(cap as usize),
            dense_data: Vec::with_capacity(cap as usize),
        }
    }

    pub fn contains(&self, i: Index) -> bool {
        (i as usize) < self.sparse.len() && {
            let dense_idx = self.sparse[i as usize] as usize;
            dense_idx < self.dense_indices.len() && self.dense_indices[dense_idx] == i
        }
    }

    pub fn insert(&mut self, i: Index, v: T) -> bool {
        let already_existed = self.contains(i);
        let dense_idx = self.dense_indices.len() as Index;
        if self.sparse.len() < (i as usize + 1) {
            // u32::MAX instead of 0 for clear value, which is slightly faster.
            // ideally i would use unsafe code to do uninitialized memory instead
            // but this is good enough for now
            self.sparse.resize(i as usize + 1, u32::MAX);
        }
        self.sparse[i as usize] = dense_idx;
        self.dense_indices.push(i);
        self.dense_data.push(v);
        already_existed
    }

    pub fn get(&self, i: Index) -> Option<&T> {
        if !self.contains(i) {
            return None;
        }
        let dense_idx = self.sparse[i as usize] as usize;
        if dense_idx < self.dense_indices.len() {
            Some(&self.dense_data[dense_idx])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, i: Index) -> Option<&mut T> {
        if !self.contains(i) {
            return None;
        }
        let dense_idx = self.sparse[i as usize] as usize;
        if dense_idx < self.dense_indices.len() {
            Some(&mut self.dense_data[dense_idx])
        } else {
            None
        }
    }

    pub fn remove(&mut self, i: Index) -> bool {
        if !self.contains(i) {
            return false;
        }

        self.dense_indices[self.sparse[i as usize] as usize] =
            self.dense_indices[self.dense_indices.len() - 1];
        self.sparse[self.dense_indices[self.dense_indices.len() - 1] as usize] =
            self.sparse[i as usize];

        self.dense_indices.pop();
        self.dense_data.pop();

        true
    }

    pub fn clear(&mut self) {
        self.dense_indices.clear();
        self.dense_data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &T)> {
        self.dense_indices
            .iter()
            .zip(self.dense_data.iter())
            .map(|v| (*v.0, v.1))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut T)> {
        self.dense_indices
            .iter()
            .zip(self.dense_data.iter_mut())
            .map(|v| (*v.0, v.1))
    }

    pub fn len(&self) -> u32 {
        self.dense_indices.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.dense_indices.is_empty()
    }
}

impl<T> Default for Pool<T> {
    fn default() -> Self {
        Self::new()
    }
}
