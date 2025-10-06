// Problem: Strongly Connected Component
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/2150
// Memory Limit: 128
// Time Limit: 2000
// Start: Mon 31 Mar 2025 03:36:12 PM KST

use std::{cmp, collections::BTreeSet};

struct TarjanSCC {
    scc: Vec<usize>,
    t_in: Vec<usize>,
    stack: Vec<usize>,

    scc_cnt: usize,
    v_cnt: usize,
}

impl TarjanSCC {
    fn set_scc(&mut self, v: usize, adj: &Vec<Vec<usize>>) -> usize {
        self.t_in[v] = self.v_cnt;
        self.v_cnt += 1;
        let mut ret: usize = self.t_in[v];
        self.stack.push(v);

        for to in &adj[v] {
            let to = *to;
            if self.t_in[to] == usize::MAX {
                ret = cmp::min(ret, self.set_scc(to, adj));
            } else if self.scc[to] == usize::MAX {
                ret = cmp::min(ret, self.t_in[to]);
            }
        }

        if ret == self.t_in[v] {
            loop {
                let t = *self.stack.last().unwrap();
                self.stack.pop();
                self.scc[t] = self.scc_cnt;
                if t == v {
                    break;
                }
            }
            self.scc_cnt += 1;
        }
        ret
    }

    fn init(&mut self, adj: &Vec<Vec<usize>>) {
        self.scc = vec![usize::MAX; adj.len()];
        self.t_in = self.scc.clone();
        for i in 0..adj.len() {
            if self.t_in[i] == usize::MAX {
                self.set_scc(i, adj);
            }
        }
    }

    fn new() -> TarjanSCC {
        TarjanSCC {
            scc: vec![],
            t_in: vec![],
            stack: vec![],

            scc_cnt: 0,
            v_cnt: 0,
        }
    }

    fn print(&self) {
        use std::fmt::Write;
        let mut stdout = String::new();

        let n: usize = self.scc.len();
        let mut visited: Vec<bool> = vec![false; n];
        let mut scc_list: Vec<BTreeSet<usize>> = vec![[0; 0].into(); n];
        writeln!(stdout, "{}", self.scc_cnt).unwrap();

        for i in 0..n {
            scc_list[self.scc[i]].insert(i);
        }

        for i in 0..n {
            if visited[i] {
                continue;
            }
            for v in &scc_list[self.scc[i]] {
                let v = *v;
                write!(stdout, "{} ", v + 1).unwrap();
                visited[v] = true;
            }
            writeln!(stdout, "{}", -1).unwrap();
        }

        print!("{stdout}");
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let v = next().parse().unwrap();
    let e = next().parse().unwrap();
    let mut adj: Vec<Vec<usize>> = vec![vec![]; v];
    for _ in 0..e {
        let x: usize = next().parse().unwrap();
        let y: usize = next().parse().unwrap();
        let x = x - 1;
        let y = y - 1;
        adj[x].push(y);
    }

    let mut tscc = TarjanSCC::new();

    tscc.init(&adj);
    tscc.print();
}
