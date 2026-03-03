use std::cmp::{max, min};

struct SegTree<T, O> {
    n_tree: usize,
    n_vec: usize,
    tree: Vec<T>,
    closure: O,
}

impl<T, O> SegTree<T, O>
where
    T: Copy,
    O: Fn(T, T) -> T,
{
    fn op(&self, l: Option<T>, r: Option<T>) -> Option<T> {
        if let Some(l_) = l {
            if let Some(r_) = r {
                Some((self.closure)(l_, r_))
            } else {
                l
            }
        } else {
            r
        }
    }

    fn new<U>(inp: &Vec<U>, t: T, closure: O) -> Self
    where
        T: Copy,
        U: Clone + Into<T>,
        O: Fn(T, T) -> T,
    {
        let n_vec = inp.len();
        let mut n_tree = 1;
        while n_tree < n_vec {
            n_tree <<= 1;
        }
        n_tree <<= 1;
        let tree = vec![t; n_tree];
        let mut s_tree = Self {
            n_tree,
            n_vec,
            tree,
            closure,
        };
        s_tree.init(inp, 1, 0, n_vec - 1);
        s_tree
    }

    fn init<U>(&mut self, inp: &Vec<U>, node: usize, l: usize, r: usize)
    where
        U: Clone + Into<T>,
    {
        if l == r {
            self.tree[node] = inp[l].clone().into();
        } else {
            let tm = (l + r) / 2;
            self.init(inp, node * 2, l, tm);
            self.init(inp, node * 2 + 1, tm + 1, r);
            self.tree[node] = Self::op(
                self,
                Some(self.tree[node * 2]),
                Some(self.tree[node * 2 + 1]),
            )
            .unwrap();
        }
    }

    fn query_(&self, node: usize, l: usize, r: usize, query_l: usize, query_r: usize) -> Option<T> {
        if query_l > query_r {
            return None;
        }
        if query_l == l && query_r == r {
            return Some(self.tree[node]);
        }
        let mid = (l + r) / 2;
        let left_query = self.query_(node * 2, l, mid, query_l, min(query_r, mid));
        let right_query = self.query_(node * 2 + 1, mid + 1, r, max(query_l, mid + 1), query_r);
        Self::op(self, left_query, right_query)
    }

    fn update_(&mut self, node: usize, l: usize, r: usize, idx: usize, val: T) {
        if l == r {
            self.tree[node] = val;
        } else {
            let mid = (l + r) / 2;
            if idx <= mid {
                self.update_(node * 2, l, mid, idx, val);
            } else {
                self.update_(node * 2 + 1, mid + 1, r, idx, val);
            }
            self.tree[node] = Self::op(
                self,
                Some(self.tree[node * 2]),
                Some(self.tree[node * 2 + 1]),
            )
            .unwrap();
        }
    }

    fn update<U>(&mut self, idx: usize, val: U)
    where
        U: Clone + Into<T>,
    {
        self.update_(1, 0, self.n_vec - 1, idx, val.into());
    }

    fn query(&self, left: usize, right: usize) -> T {
        self.query_(1, 0, self.n_vec - 1, left, right).unwrap()
    }
}
