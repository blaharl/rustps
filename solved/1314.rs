// Problem: 동굴 탐험
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/1314
// Memory Limit: 128
// Time Limit: 2000
// Start: Tue 01 Apr 2025 09:26:00 AM KST

use std::{
    cmp::{self, Reverse},
    collections::BinaryHeap,
};

const INF: usize = 10_usize.pow(18);

struct Graph {
    walk_time: Vec<usize>,
    weight: Vec<usize>,
    trust: Vec<usize>,
    b: usize,
    valid: usize,

    dist: Vec<usize>,
    dist_pass: Vec<usize>,
}

impl Graph {
    // O(N)
    fn get_time(&mut self, group: usize) -> usize {
        if group.count_ones() == 1 {
            let member = group.trailing_zeros() as usize;
            if self.weight[member] <= self.b {
                return self.walk_time[member];
            } else {
                return usize::MAX;
            }
        }
        let mut grp = group;
        let mut wgt = 0_usize;
        let mut t = 0_usize;
        while grp != 0 {
            let member = grp.trailing_zeros() as usize;
            grp &= !(1 << member);

            if (self.trust[member] & group) == 0 {
                return usize::MAX;
            }
            wgt += self.weight[member];
            if wgt > self.b {
                return usize::MAX;
            }
            t = cmp::max(t, self.walk_time[member]);
        }
        t
    }

    // O(|E|lg|E|) = O(|E|lg|V|)
    fn dijkstra(&mut self, src: usize) {
        self.dist[src] = 0;

        // greater?, (cost, passed set)
        let mut pq = BinaryHeap::new();
        //pq.push((0, src));
        pq.push(Reverse((0, src)));

        while !pq.is_empty() {
            let Reverse((cost, curr)) = pq.pop().unwrap();

            if self.dist[curr] < cost {
                continue;
            }

            let left = (!curr) & self.valid;

            let mut grp_leave = left;
            while grp_leave != 0 {
                let t_leave = self.get_time(grp_leave);
                if t_leave == usize::MAX {
                    grp_leave = (grp_leave - 1) & left;
                    continue;
                }
                let passed = curr | grp_leave;
                if self.dist_pass[passed] <= cost + t_leave {
                    grp_leave = (grp_leave - 1) & left;
                    continue;
                }
                self.dist_pass[passed] = cost + t_leave;
                if passed == self.valid {
                    if self.dist[passed] > cost + t_leave {
                        self.dist[passed] = cost + t_leave;
                    }
                    grp_leave = (grp_leave - 1) & left;
                    continue;
                }

                let mut grp_return = passed;
                while grp_return != 0 {
                    let t_return = self.get_time(grp_return);
                    if t_return == usize::MAX {
                        grp_return = (grp_return - 1) & passed;
                        continue;
                    }
                    let t = t_leave + t_return;
                    let next = passed & !grp_return;
                    if self.dist[next] > cost + t {
                        self.dist[next] = cost + t;
                        pq.push(Reverse((cost + t, next)));
                    }
                    grp_return = (grp_return - 1) & passed;
                }
                grp_leave = (grp_leave - 1) & left;
            }
        }
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let n: usize = next().parse().unwrap();
    let mut graph = Graph {
        valid: (1 << n) - 1,
        dist: vec![INF; 1 << n],
        dist_pass: vec![INF; 1 << n],
        walk_time: vec![0; n],
        weight: vec![0; n],
        trust: vec![0; n],
        b: 0,
    };

    for i in 0..n {
        graph.weight[i] = next().parse().unwrap();
        graph.walk_time[i] = next().parse().unwrap();
    }

    for i in 0..n {
        let c: String = next().parse().unwrap();
        for j in 0..n {
            if i != j && c.chars().nth(j).unwrap() == 'Y' {
                graph.trust[i] |= 1_usize << j;
            }
        }
    }

    graph.b = next().parse().unwrap();

    graph.dijkstra(0);

    if graph.dist[graph.valid as usize] == INF {
        println!("-1");
    } else {
        println!("{}", graph.dist[graph.valid as usize]);
    }
}
