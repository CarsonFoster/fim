use core::cmp::{max, Ordering};

// [1]: http://people.csail.mit.edu/rivest/pubs/GR93.pdf
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
        } if node > self.tree.len() {
            self.tree.reserve(node - self.tree.len()); // may reserve more than necessary to prevent future reallocations
            self.tree.resize_with(node + 1, || None); // fill new places with None, new len = node + 1
        }
        self.tree[node] = Some(new);
        self.size += 1;
        self.max_size = max(self.size, self.max_size);

        if depth >= self.deep_height() {
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
    }

    fn rebuild(&mut self, scapegoat: usize, subtree_size: Option<usize>) {
        let (mut sorted_subtree, mut stack) = if let Some(size) = subtree_size {
            (Vec::with_capacity(size as usize), Vec::with_capacity(size as usize))
        } else {
            (Vec::new(), Vec::new())
        };
        let mut node_idx = scapegoat;

        while !stack.is_empty() || self.is_valid(node_idx) {
            if self.is_valid(node_idx) {
                stack.push(node_idx);
                node_idx = left(node_idx);
            } else {
                node_idx = stack.pop().expect("stack should never be empty here");
                sorted_subtree.push(self.tree[node_idx].take().expect("node_idx should be valid"));
                node_idx = right(node_idx);
            }
        }
    }

    fn is_valid(&self, idx: usize) -> bool {
        (idx as usize) < self.tree.len() && self.tree[idx].is_some()
    }

    fn size(&self, root: usize) -> usize {
        let mut stack = Vec::new();
        let mut size = 0;
        stack.push(root);
        while let Some(node) = stack.pop() {
            if self.tree[node].is_some() {
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
