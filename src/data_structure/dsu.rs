// Problem: 교수님은 기다리지 않는다
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/3830
// Memory Limit: 256
// Time Limit: 2000
// Start: Tue 08 Apr 2025 11:46:19 PM KST

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

fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let t = next().parse().unwrap();
    for _ in 0..t {
        let a: i32 = next().parse().unwrap();
        let b: i32 = next().parse().unwrap();
        writeln!(stdout, "{}", a + b).unwrap();
    }
    print!("{stdout}");
}
