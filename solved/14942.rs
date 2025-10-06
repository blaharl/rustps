// Problem: 개미
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/14942
// Memory Limit: 256
// Time Limit: 2000
// Start: Sat 12 Apr 2025 01:54:10 PM KST

use std::cmp;

#[derive(Debug)]
struct Graph {
    adj: Vec<Vec<(usize, u64)>>,
    parent: Vec<Vec<(usize, u64)>>,
    max_depth: usize,
    log_depth: usize,
}

impl Graph {
    fn dfs(&mut self, node: usize, par: usize, depth: usize) {
        self.max_depth = cmp::max(self.max_depth, depth);
        for (v, cost) in &self.adj[node].clone() {
            let (v, cost) = (*v, *cost);
            if v == par {
                continue;
            }
            self.parent[0][v] = (node, cost);
            {
                self.dfs(v, node, depth + 1);
            }
        }
    }

    fn get_room(&self, node: usize, mut energy: u64) -> usize {
        if node == 0 {
            return 0;
        }
        for d in (0..self.log_depth).rev() {
            if self.parent[d][node].1 > energy {
                continue;
            }
            energy -= self.parent[d][node].1;
            return self.get_room(self.parent[d][node].0, energy);
        }
        node
    }
}

fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let n = next().parse().unwrap();
    let mut energy = vec![0_u64; n];
    for i in 0..n {
        energy[i] = next().parse().unwrap();
    }
    let mut graph = Graph {
        adj: vec![vec![(0, 0); 0]; n],
        parent: vec![vec![(0, 0); n]; 1],
        max_depth: 0,
        log_depth: 0,
    };

    for _ in 1..n {
        let u: usize = next().parse().unwrap();
        let v: usize = next().parse().unwrap();
        let e: u64 = next().parse().unwrap();
        let u = u - 1;
        let v = v - 1;
        graph.adj[u].push((v, e));
        graph.adj[v].push((u, e));
    }

    graph.dfs(0, 0, 0);
    let md = graph.max_depth as f64;
    graph.log_depth = md.log2().ceil() as usize;

    graph.parent.resize(graph.log_depth, vec![(0, 0); n]);

    for i in 0..graph.log_depth - 1 {
        for j in 0..n {
            let pp = graph.parent[i][graph.parent[i][j].0];
            graph.parent[i + 1][j].0 = pp.0;
            graph.parent[i + 1][j].1 = graph.parent[i][j].1 + pp.1;
        }
    }

    for i in 0..n {
        writeln!(stdout, "{}", graph.get_room(i, energy[i]) + 1).unwrap();
    }

    print!("{stdout}");
}
