// Problem: 세금
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/13907
// Memory Limit: 256
// Time Limit: 2000
// Start: Sat 11 Oct 2025 12:43:33 PM KST

use std::cmp::min;
#[allow(unused_imports)]
use std::io::{BufWriter, Write, stdin, stdout};
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

const INF: u64 = 1e18 as u64;

fn dijkstra(src: usize, adj: &[Vec<(u32, usize)>]) -> Vec<Vec<u64>> {
    let v = adj.len();
    // [node][count] -> cost
    let mut dist = vec![vec![INF; v]; v];
    dist[src][0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, 0, src)));

    while let Some(Reverse((cost, cnt, curr))) = pq.pop() {
        if dist[curr][cnt] < cost {
            continue;
        }
        if cnt == v - 1 {
            continue;
        }

        // adj : (cost, node)
        for next in &adj[curr] {
            if dist[next.1][cnt + 1] > next.0 as u64 + cost {
                dist[next.1][cnt + 1] = next.0 as u64 + cost;
                pq.push(Reverse((next.0 as u64 + cost, cnt + 1, next.1)));
            }
        }
    }

    dist
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let (v, e) = (scan.next(), scan.next());
    let mut adj: Vec<Vec<(u32, usize)>> = vec![vec![]; v];

    let k = scan.next();

    let (s, d) = (scan.next::<usize>() - 1, scan.next::<usize>() - 1);

    for _ in 0..e {
        let (src, dst, wgt) = (
            scan.next::<usize>() - 1,
            scan.next::<usize>() - 1,
            scan.next::<u32>(),
        );
        adj[src].push((wgt, dst));
        adj[dst].push((wgt, src));
    }

    let dists = &dijkstra(s, &adj)[d];
    let mut min_cost = INF;
    for d in dists {
        min_cost = min(*d, min_cost);
    }
    writeln!(out, "{}", min_cost).ok();

    let mut tax = 0_u64;

    for _ in 0..k {
        let tax_add = scan.next::<u64>();
        tax += tax_add;
        min_cost = INF;
        for i in 0..v {
            min_cost = min(min_cost, dists[i] + i as u64 * tax);
        }
        writeln!(out, "{}", min_cost).ok();
    }
}
