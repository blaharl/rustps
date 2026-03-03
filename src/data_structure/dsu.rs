use std::mem::swap;

struct DisjointSet {
    root: Vec<usize>,
    height: Vec<u32>,
}

impl DisjointSet {
    fn build(n: usize) -> Self {
        let mut dsu = DisjointSet {
            root: vec![0; n],
            height: vec![0; n],
        };

        for i in 0..n {
            dsu.root[i] = i;
        }

        dsu
    }
    fn find_set(&mut self, u: usize) -> usize {
        if u == self.root[u] {
            return u;
        }
        let r = self.find_set(self.root[u]);
        self.root[u] = r;
        r
    }

    fn union_set(&mut self, u: usize, v: usize) {
        let mut u = self.find_set(u);
        let mut v = self.find_set(v);

        if u == v {
            return;
        }

        if self.height[u] > self.height[v] {
            swap(&mut u, &mut v);
        }

        if self.height[u] == self.height[v] {
            self.height[v] += 1;
        }

        self.root[u] = v;
    }
}
