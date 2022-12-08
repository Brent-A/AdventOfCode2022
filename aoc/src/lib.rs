use std::collections::HashMap;
pub mod grid;
pub mod position;

pub fn load_input(base: &str, path: &str) -> String {
    let path = std::path::Path::new(base).join(path);

    std::fs::read_to_string(path).unwrap()
}

pub trait GetOrDefault<K, T>
where
    T: Default,
{
    fn get_or_default(&mut self, index: K) -> &T;
    fn get_mut_or_default(&mut self, index: K) -> &mut T;
}

impl<T> GetOrDefault<usize, T> for Vec<T>
where
    T: Default,
{
    fn get_or_default(&mut self, index: usize) -> &T {
        while self.len() < index + 1 {
            self.push(T::default());
        }
        self.get(index).unwrap()
    }
    fn get_mut_or_default(&mut self, index: usize) -> &mut T {
        while self.len() < index + 1 {
            self.push(T::default());
        }
        self.get_mut(index).unwrap()
    }
}
impl<K, T> GetOrDefault<K, T> for HashMap<K, T>
where
    T: Default,
    K: Eq + std::hash::Hash + Clone,
{
    fn get_or_default(&mut self, index: K) -> &T {
        if !self.contains_key(&index) {
            self.insert(index.clone(), T::default());
        }
        self.get(&index).unwrap()
    }

    fn get_mut_or_default(&mut self, index: K) -> &mut T {
        if !self.contains_key(&index) {
            self.insert(index.clone(), T::default());
        }
        self.get_mut(&index).unwrap()
    }
}
