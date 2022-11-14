use core::cmp::{max, Ordering};
use std::ops::Index;

// [1]: http://people.csail.mit.edu/rivest/pubs/GR93.pdf
// [2]: https://en.wikipedia.org/wiki/Scapegoat_tree
// "Unlike most other self-balancing search trees, scapegoat trees are entirely flexible as to their
// balancing. They support any α such that 0.5 < α < 1. A high α value results in fewer balances,
// making insertion quicker but lookups and deletions slower, and vice versa for a low α. Therefore
// in practical applications, an α can be chosen depending on how frequently these actions should
// be performed." ([2])
pub struct ScapegoatTree<T> {
    tree: Vec<Option<T>>,
    alpha_reciprocal: f32,
    size: usize,
    max_size: usize
}

impl<T> ScapegoatTree<T> {
    const ROOT: usize = 1;

    pub fn new(alpha: f32) -> Self {
        ScapegoatTree{ tree: Vec::new(), size: 0, max_size: 0, alpha_reciprocal: (1.0 / alpha) }
    }

    pub fn delete<R: AsRef<T>>(&mut self, item: R) -> Option<T>
    where
        T: Ord
    {
        let item = item.as_ref();
        self.delete_with(|tree_el| item.cmp(tree_el))
    }

    pub fn delete_with<F>(&mut self, f: F) -> Option<T>
    where
        F: FnMut(&T) -> Ordering
    {
        if let Some(idx) = self.idx_search_with(f) {
            let (left_valid, right_valid) = (self.is_valid(left(idx)), self.is_valid(right(idx)));
            let el = if !left_valid && !right_valid {
                // has no children, can take directly
                self.tree[idx].take()
            } else if left_valid != right_valid {
                // has one child, swap and then take
                let child = if left_valid { left(idx) } else { right(idx) };
                self.tree.swap(idx, child);
                self.tree[child].take()
            } else {
                // has two children, do hibbard deletion
                // try to do predecessor/successor about half each
                // not guaranteed, but tree rebalances itself anyway
                let new_node = if idx % 2 == 0 {
                    // prececessor
                    let mut node = left(idx);
                    let mut right_ = right(node);
                    while self.is_valid(right_) {
                        node = right_;
                        right_ = right(node);
                    }
                    let left_ = left(node);
                    if self.is_valid(left_) {
                        self.tree.swap(node, left_);
                        left_
                    } else {
                        node
                    }
                } else {
                    // successor
                    let mut node = right(idx);
                    let mut left_ = left(node);
                    while self.is_valid(left_) {
                        node = left_;
                        left_ = left(node);
                    }
                    let right_ = right(node);
                    if self.is_valid(right_) {
                        self.tree.swap(node, right_);
                        right_
                    } else {
                        node
                    }
                };
                self.tree.swap(new_node, idx);
                self.tree[new_node].take()
            };
            self.size -= 1;
            let alpha = 1.0 / self.alpha_reciprocal;
            if (self.size as f32) < alpha * (self.max_size as f32) {
                self.rebuild(Self::ROOT, Some(self.size));
                self.max_size = self.size;
            }
            return el;
        }
        None
    }

    pub fn search<R: AsRef<T>>(&self, item: R) -> Option<&T>
    where
        T: Ord
    {
        let item = item.as_ref();
        self.search_with(|tree_el| item.cmp(tree_el))
    }

    pub fn search_with<F>(&self, f: F) -> Option<&T>
    where
        F: FnMut(&T) -> Ordering
    {
        self.idx_search_with(f).map(|idx| self.tree.index(idx).as_ref().expect("idx holds node"))
    }

    pub fn search_mut<R: AsRef<T>>(&mut self, item: R) -> Option<&mut T>
    where
        T: Ord
    {
        let item = item.as_ref();
        self.search_with_mut(|tree_el| item.cmp(tree_el))
    }

    pub fn search_with_mut<F>(&mut self, f: F) -> Option<&mut T>
    where
        F: FnMut(&T) -> Ordering
    {
        self.idx_search_with(f).map(|idx| self.tree.get_mut(idx).expect("idx is in bounds").as_mut().expect("idx holds node"))
    }

    pub fn get(&self, mut rank: usize) -> Option<&T> {
        if rank >= self.size {
            return None;
        }

        let idx = self.get_recursive(Self::ROOT, &mut rank).expect("valid rank should yield valid index");
        Some(self.tree.index(idx).as_ref().expect("idx holds node"))
    }

    pub fn get_mut(&mut self, mut rank: usize) -> Option<&mut T> {
        if rank >= self.size {
            return None;
        }

        let idx = self.get_recursive(Self::ROOT, &mut rank).expect("valid rank should yield valid index");
        Some(self.tree.get_mut(idx).expect("idx is in bounds").as_mut().expect("idx holds node"))
    }

    pub fn insert_rank(&mut self, mut rank: usize, new: T) {
        if rank > self.size {
            return;
        }
        let idx = self.insert_rank_recursive(Self::ROOT, &mut rank).expect("valid rank should yield valid index");
        self.put(idx, new);
        self.size += 1;
        self.max_size = max(self.size, self.max_size);

        if log2(idx) >= self.deep_height() {
            self.scapegoat(idx);
        }
    }

    pub fn insert(&mut self, new: T)
    where
        T: Ord
    {
        let mut node = Self::ROOT;
        let mut depth = 0;
        while let Some(el) = self.tree.get(node).map(|o| o.as_ref()).flatten() {
            match new.cmp(el) {
                Ordering::Less => node = left(node),
                Ordering::Greater => node = right(node),
                Ordering::Equal => return // TODO: check if this is what we want
            };
            depth += 1;
        }
        self.put(node, new);
        self.size += 1;
        self.max_size = max(self.size, self.max_size);

        if depth >= self.deep_height() {
            self.scapegoat(node);
        }
    }

    fn get_recursive(&self, idx: usize, rank: &mut usize) -> Option<usize> {
        if !self.is_valid(idx) {
            return None;
        }

        if let ret @ Some(_) = self.get_recursive(left(idx), rank) {
            return ret;
        }
        if *rank == 0 {
            return Some(idx);
        }

        *rank -= 1;

        self.get_recursive(right(idx), rank)
    }

    fn insert_rank_recursive(&self, idx: usize, rank: &mut usize) -> Option<usize> {
        if !self.is_valid(idx) {
            return None;
        }

        let left_ = left(idx);
        if let ret @ Some(_) = self.insert_rank_recursive(left_, rank) {
            return ret;
        }
        if *rank == 0 {
            return Some(left_);
        }

        *rank -= 1;

        let right_ = right(idx);
        if let ret @ Some(_) = self.insert_rank_recursive(right_, rank) {
            return ret;
        }
        if *rank == 0 {
            Some(right_)
        } else {
            None
        }
    }

    fn idx_search_with<F>(&self, mut f: F) -> Option<usize>
    where
        F: FnMut(&T) -> Ordering
    {
        let mut node = Self::ROOT;
        while let Some(el) = self.tree.get(node).map(|o| o.as_ref()).flatten() {
            match f(el) {
                Ordering::Less => node = left(node),
                Ordering::Greater => node = right(node),
                Ordering::Equal => return Some(node)
            };
        }
        None
    }

    fn scapegoat(&mut self, mut node: usize) {
        let mut i = 0; // 0 = current node, i + 1 = parent of i
        let mut size = 1; // size of current node
        let mut size_sibling = self.size(sibling(node)); // size of current node's sibling (other child of this node's parent)
        loop {
            node /= 2; // traverse to parent
            i += 1; // increment reverse depth / parent distance
            size = 1 + size + size_sibling;
            if i > self.h_alpha(size) {
                // always satisfied by root, according to [1]
                // and using this criteria may result in more balanced trees on average
                self.rebuild(node, Some(size));
                break;
            }
            size_sibling = self.size(sibling(node));
        }
    }

    fn rebuild(&mut self, scapegoat: usize, subtree_size: Option<usize>) {
        let mut sorted_subtree = self.pull_subtree(scapegoat, subtree_size);
        if !sorted_subtree.is_empty() {
            self.put_subtree(scapegoat, 0, sorted_subtree.len() - 1, &mut sorted_subtree);
        }
    }

    fn put_subtree(&mut self, idx: usize, lo: usize, hi: usize, subtree: &mut Vec<Option<T>>) {
        let m = median(lo, hi);
        self.put(idx, subtree[m].take().expect("subtree should only contain valid values"));
        if lo != hi {
            self.put_subtree(left(idx), lo, m - 1, subtree);
            self.put_subtree(right(idx), m + 1, hi, subtree);
        }
    }

    fn pull_subtree(&mut self, idx: usize, subtree_size: Option<usize>) -> Vec<Option<T>> {
        let mut sorted_subtree = if let Some(size) = subtree_size { Vec::with_capacity(size) } else { Vec::new() };
        self.inorder(idx, &mut sorted_subtree);
        sorted_subtree
    }

    fn inorder(&mut self, idx: usize, subtree: &mut Vec<Option<T>>) {
        if self.is_valid(idx) {
            self.inorder(left(idx), subtree);
            subtree.push(self.tree[idx].take());
            self.inorder(right(idx), subtree);
        }
    }

    fn put(&mut self, idx: usize, value: T) {
        if idx >= self.tree.len() {
            self.tree.reserve(idx - self.tree.len() + 1); // may reserve more than necessary to prevent future reallocations
            self.tree.resize_with(idx + 1, || None); // fill new places with None, new len = idx + 1
        }
        self.tree[idx] = Some(value);
    }

    fn is_valid(&self, idx: usize) -> bool {
        idx < self.tree.len() && self.tree[idx].is_some()
    }

    fn size(&self, root: usize) -> usize {
        let mut stack = Vec::new();
        let mut size = 0;
        stack.push(root);
        while let Some(node) = stack.pop() {
            if self.is_valid(node) {
                size += 1;
                stack.push(left(node));
                stack.push(right(node));
            }
        }
        size
    }

    fn deep_height(&self) -> usize {
        self.h_alpha(self.size)
    }

    fn h_alpha(&self, n: usize) -> usize {
        // h_alpha(n) = ⌊log_(1/alpha) (n)⌋
        f32::log(n as f32, self.alpha_reciprocal).floor() as usize
    }
}

// both inclusive
const fn median(left: usize, right: usize) -> usize {
    (left + right) >> 1
}

const fn left(parent: usize) -> usize {
    2*parent
}

const fn right(parent: usize) -> usize {
    2*parent + 1
}

const fn sibling(child: usize) -> usize {
    // children are 2n and 2n + 1
    // so sibling is +1 if even, and -1 if odd
    if child % 2 == 0 {
        child + 1
    } else {
        child - 1
    }
}

const fn log2(mut n: usize) -> usize {
    let mut count = 0;
    while n > 0 {
        n >>= 1;
        count += 1;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::ScapegoatTree;
    use std::cmp::Ordering;

    struct TestStruct {
        comp: usize,
        non_comp: usize
    }

    impl PartialEq for TestStruct {
        fn eq(&self, other: &TestStruct) -> bool {
            self.comp == other.comp
        }
    }

    impl Eq for TestStruct {}

    impl PartialOrd for TestStruct {
        fn partial_cmp(&self, other: &TestStruct) -> Option<Ordering> {
            self.comp.partial_cmp(&other.comp)
        }
    }

    impl Ord for TestStruct {
        fn cmp(&self, other: &TestStruct) -> Ordering {
            self.comp.cmp(&other.comp)
        }
    }

    fn setup() -> ScapegoatTree<TestStruct> {
        let mut tree = ScapegoatTree::new(0.75);
        for i in 0..100 {
            let el = TestStruct{ comp: i, non_comp: 100 - i };
            tree.insert(el);
        }
        tree
    }

    #[test]
    pub fn test_get() {
        let tree = setup();
        for i in 0..100 {
            let el = tree.get(i).unwrap();
            assert!(el.comp == i && el.non_comp == 100 - i);
        }
    }

    #[test]
    pub fn test_get_none() {
        let tree = setup();
        assert!(tree.get(100).is_none());
    }

    #[test]
    pub fn test_get_mut() {
        let mut tree = setup();
        for i in 0..100 {
            let mut el = tree.get_mut(i).unwrap();
            el.non_comp = i * 2;
        }
        for i in 0..100 {
            let el = tree.get(i).unwrap();
            assert!(el.comp == i && el.non_comp == i * 2);
        }
    }

    #[test]
    pub fn test_get_mut_none() {
        let mut tree = setup();
        assert!(tree.get_mut(101).is_none());
    }
}
