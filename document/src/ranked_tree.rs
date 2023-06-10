use std::{borrow::Borrow, cmp::Ordering};

pub trait RankedTree<T> {
    fn delete<R: Borrow<T>>(&mut self, item: R) -> Option<T>
    where
        T: Ord;

    fn delete_with<F>(&mut self, f: F) -> Option<T>
    where
        F: FnMut(&T) -> Ordering;

    fn search<R: Borrow<T>>(&self, item: R) -> Option<&T>
    where
        T: Ord;

    fn search_with<F>(&self, f: F) -> Option<&T>
    where
        F: FnMut(&T) -> Ordering;

    fn search_mut<R: Borrow<T>>(&mut self, item: R) -> Option<&mut T>
    where
        T: Ord;

    fn search_with_mut<F>(&mut self, f: F) -> Option<&mut T>
    where
        F: FnMut(&T) -> Ordering;

    fn get(&self, rank: usize) -> Option<&T>;

    fn get_mut(&mut self, rank: usize) -> Option<&mut T>;

    fn insert_rank(&mut self, rank: usize, new: T);

    fn insert(&mut self, new: T)
    where
        T: Ord;
}
