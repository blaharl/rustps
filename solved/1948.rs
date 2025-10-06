// Problem: 임계경로
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/1948
// Memory Limit: 512
// Time Limit: 2000
// Start: Sat 29 Mar 2025 12:21:58 PM KST

use std::collections::{HashSet, VecDeque};

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let n: usize = next().parse().unwrap();
    let m: usize = next().parse().unwrap();
    let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
    let mut rev: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
    let mut dep: Vec<usize> = vec![0; n];
    let mut max_time: Vec<i32> = vec![0; n];
    for _ in 0..m {
        let s: usize = next().parse().unwrap();
        let e: usize = next().parse().unwrap();
        let w: i32 = next().parse().unwrap();
        let s = s - 1;
        let e = e - 1;
        adj[s].push((e, w));
        rev[e].push((s, w));
        dep[e] += 1;
    }
    let s: usize = next().parse().unwrap();
    let e: usize = next().parse().unwrap();
    let s = s - 1;
    let e = e - 1;

    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(s);

    while !q.is_empty() {
        let curr = *q.front().unwrap();
        let cost = max_time[curr];
        q.pop_front();

        for (next, c) in &adj[curr] {
            let next = *next;
            dep[next] -= 1;
            if dep[next] == 0 {
                q.push_back(next);
            }

            let next_cost = cost + c;

            if max_time[next] > next_cost {
                continue;
            }

            if max_time[next] < next_cost {
                rev[next].clear();
                max_time[next] = next_cost;
            }
            rev[next].push((curr, *c));
        }
    }

    println!("{}", max_time[e]);

    let mut visited_edge: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: Vec<bool> = vec![false; n];

    let mut cnt: u32 = 0;
    q.push_back(e);

    while !q.is_empty() {
        let curr = *q.front().unwrap();
        q.pop_front();

        for (next, _) in &rev[curr] {
            let next = *next;
            if visited_edge.contains(&(curr, next)) {
                continue;
            }
            visited_edge.insert((curr, next));
            cnt += 1;
            if !visited[next] {
                q.push_back(next);
                visited[next] = true;
            }
        }
    }

    println!("{}", cnt);
}
