// Problem: LCA 2
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/11438
// Memory Limit: 256
// Time Limit: 1500
// Start: Sun 13 Apr 2025 03:21:18 PM KST

use std::{cmp, mem::swap};

#[derive(Debug)]
struct Graph {
    adj: Vec<Vec<usize>>,
    parent: Vec<Vec<usize>>,
    depth: Vec<usize>,
    max_depth: usize,
    log_depth: usize,
}

impl Graph {
    fn build(size: usize) -> Graph {
        Graph {
            adj: vec![vec![0; 0]; size],
            parent: vec![vec![0; size]; 1],
            depth: vec![0; size],
            max_depth: 0,
            log_depth: 0,
        }
    }
    fn dfs(&mut self, node: usize, par: usize, depth: usize) {
        self.max_depth = cmp::max(self.max_depth, depth);
        self.depth[node] = depth;
        for v in &self.adj[node].clone() {
            let v = *v;
            if v == par {
                continue;
            }
            self.parent[0][v] = node;
            self.dfs(v, node, depth + 1);
        }
    }

    fn build_table(&mut self, root: usize) {
        self.dfs(root, root, 0);
        let md = self.max_depth as f64;
        self.log_depth = md.log2().ceil() as usize;

        let n = self.adj.len();

        self.parent.resize(self.log_depth, vec![0; n]);

        for i in 0..self.log_depth - 1 {
            for j in 0..n {
                self.parent[i + 1][j] = self.parent[i][self.parent[i][j]];
            }
        }
    }

    fn get_lca(&self, u: usize, v: usize) -> usize {
        if u == v {
            return u;
        }

        let mut u = u;
        let mut v = v;

        if self.depth[u] != self.depth[v] {
            if self.depth[u] < self.depth[v] {
                swap(&mut u, &mut v);
            }
            for l in (0..self.log_depth).rev() {
                if self.depth[self.parent[l][u]] < self.depth[v] {
                    continue;
                }
                return self.get_lca(self.parent[l][u], v);
            }
        }

        for l in (0..self.log_depth).rev() {
            let pu = self.parent[l][u];
            let pv = self.parent[l][v];
            if pu == pv {
                continue;
            }
            return self.get_lca(pu, pv);
        }

        self.get_lca(self.parent[0][u], self.parent[0][v])
    }
}

fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let n = next().parse().unwrap();
    let mut graph = Graph::build(n);

    for _ in 1..n {
        let u: usize = next().parse().unwrap();
        let v: usize = next().parse().unwrap();
        let u = u - 1;
        let v = v - 1;
        graph.adj[u].push(v);
        graph.adj[v].push(u);
    }

    graph.build_table(0);

    let m = next().parse().unwrap();
    for _ in 0..m {
        let u: usize = next().parse().unwrap();
        let v: usize = next().parse().unwrap();
        let u = u - 1;
        let v = v - 1;
        let lca = graph.get_lca(u, v);
        writeln!(stdout, "{}", lca + 1).unwrap();
    }

    print!("{stdout}");
}
