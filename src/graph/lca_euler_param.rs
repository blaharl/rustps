// Problem: LCA 2
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/11438
// Memory Limit: 256
// Time Limit: 1500
// Start: Sun 13 Apr 2025 03:21:18 PM KST

#[derive(Debug)]
struct Graph {
    timer: usize,
    parent: Vec<Vec<usize>>,
    tin: Vec<usize>,
    tout: Vec<usize>,
    lg_depth: usize,
}

impl Graph {
    fn build(size: usize) -> Graph {
        let lg = (size as f64).log2().ceil();
        Graph {
            timer: 0,
            parent: vec![vec![0; lg as usize + 1]; size],
            tin: vec![0; size],
            tout: vec![0; size],
            lg_depth: lg as usize,
        }
    }

    fn build_anc(&mut self, root: usize, adj: &Vec<Vec<usize>>) {
        self.dfs(root, root, adj);
    }

    fn dfs(&mut self, node: usize, par: usize, adj: &Vec<Vec<usize>>) {
        self.timer += 1;
        self.tin[node] = self.timer;
        self.parent[node][0] = par;

        for i in 1..=self.lg_depth {
            self.parent[node][i] = self.parent[self.parent[node][i - 1]][i - 1];
        }

        for next in &adj[node] {
            let next = *next;
            if next != par {
                self.dfs(next, node, adj);
            }
        }

        self.timer += 1;
        self.tout[node] = self.timer;
    }

    fn is_ancestor(&self, u: usize, v: usize) -> bool {
        self.tin[u] <= self.tin[v] && self.tout[u] >= self.tout[v]
    }

    fn get_lca(&self, u: usize, v: usize) -> usize {
        if self.is_ancestor(u, v) {
            return u;
        }
        if self.is_ancestor(v, u) {
            return v;
        }

        let mut u = u;

        for i in (0..=self.lg_depth).rev() {
            if !self.is_ancestor(self.parent[u][i], v) {
                u = self.parent[u][i];
            }
        }

        self.parent[u][0]
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
    let mut adj = vec![vec![0_usize; 0]; n];

    for _ in 1..n {
        let u: usize = next().parse().unwrap();
        let v: usize = next().parse().unwrap();
        let u = u - 1;
        let v = v - 1;
        adj[u].push(v);
        adj[v].push(u);
    }

    graph.build_anc(0, &adj);

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
