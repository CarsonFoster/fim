use std::cmp::Ordering;
use crate::ranked_tree::RankedTree;

pub struct RBTree<T>
{
    #[doc(hidden)]
    root: Option<Box<RBNode<T>>>,
}

struct RBNode<T>
{
    value: T,
    left: Option<Box<RBNode<T>>>,
    right: Option<Box<RBNode<T>>>,
    red: bool,
}

impl<T> RankedTree<T> for RBTree<T>
{
    fn delete<R: std::borrow::Borrow<T>>(&mut self, item: R) -> Option<T>
        where T: Ord
    {
        todo!()
    }

    fn delete_with<F>(&mut self, f: F) -> Option<T>
        where F: FnMut(&T) -> Ordering
    {
        todo!()
    }

    fn search<R: std::borrow::Borrow<T>>(&self, item: R) -> Option<&T>
        where T: Ord
    {
        let ptr = item.borrow();
        self.search_with(|el| ptr.cmp(el))
    }

    fn search_with<F>(&self, f: F) -> Option<&T>
        where F: FnMut(&T) -> Ordering 
    {
        self.node_search_with(f).map(|node| &node.value)
    }

    fn search_mut<R: std::borrow::Borrow<T>>(&mut self, item: R) -> Option<&mut T>
        where T: Ord
    { 
        let ptr = item.borrow();
        self.search_with_mut(|el| ptr.cmp(el))
    }

    fn search_with_mut<F>(&mut self, f: F) -> Option<&mut T>
        where F: FnMut(&T) -> Ordering 
    {
        self.node_search_with_mut(f).map(|node| &mut node.value)
    }

    fn get(&self, mut rank: usize) -> Option<&T> {
        let mut node_opt: Option<&Box<RBNode<T>>> = self.root.as_ref();

        None
    }

    fn get_mut(&mut self, rank: usize) -> Option<&mut T>
    {
        todo!()
    }

    fn insert_rank(&mut self, rank: usize, new: T)
    {
        todo!()
    }

    fn insert(&mut self, new: T)
        where T: Ord 
    {
        todo!()
    }
}

impl<T> RBNode<T>
{
    fn new(value: T) -> Self
    {
        Self { value, left: None, right: None, red: false }
    }

    fn rotate_left(mut node: Box<RBNode<T>>) -> Box<RBNode<T>>
    {
        let mut right = node.right.unwrap();
        node.right = right.left.take();
        right.red = node.red;
        node.red = true;
        right.left = Some(node);
        right
    }

    fn rotate_right(mut node: Box<RBNode<T>>) -> Box<RBNode<T>>
    {
        let mut left = node.left.unwrap();
        node.left = left.right.take();
        left.red = node.red;
        node.red = true;
        left.right = Some(node);
        left
    }
}

impl<T> RBTree<T>
{
    pub fn new() -> Self
    {
        Self { root: None }
    }

    fn node_search_with<F>(&self, mut f: F) -> Option<&RBNode<T>>
        where F: FnMut(&T) -> Ordering
    {
        let mut node_opt: Option<&Box<RBNode<T>>> = self.root.as_ref();
        while let Some(node) = node_opt {
            match f(&node.value) {
                Ordering::Less => node_opt = node.left.as_ref(),
                Ordering::Equal => return Some(node.as_ref()),
                Ordering::Greater => node_opt = node.right.as_ref(),
            }
        }
        None
    }

    fn node_search_with_mut<F>(&mut self, mut f: F) -> Option<&mut RBNode<T>>
        where F: FnMut(&T) -> Ordering
    {
        let mut node_opt: Option<&mut Box<RBNode<T>>> = self.root.as_mut();
        while let Some(node) = node_opt {
            match f(&node.value) {
                Ordering::Less => node_opt = node.left.as_mut(),
                Ordering::Equal => return Some(node.as_mut()),
                Ordering::Greater => node_opt = node.right.as_mut(),
            }
        }
        None
    }
}
