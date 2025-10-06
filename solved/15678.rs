// Problem: 연세워터파크
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/15678
// Memory Limit: 128
// Time Limit: 1000
// Start: Fri 28 Mar 2025 07:39:56 PM KST

use std::{cmp, collections::VecDeque};

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let n: usize = next().parse().unwrap();
    let d: usize = next().parse().unwrap();
    let mut deque: VecDeque<(i64, usize)> = VecDeque::new();
    let mut sum: i64 = i32::MIN.into();
    for i in 0..n {
        let mut ki: i64 = next().parse().unwrap();
        while !deque.is_empty() && deque.front().unwrap().1 + d < i {
            deque.pop_front();
        }
        if !deque.is_empty() && deque.front().unwrap().0 > 0 {
            ki += deque.front().unwrap().0;
        }
        sum = cmp::max(sum, ki);
        while !deque.is_empty() && deque.back().unwrap().0 <= ki {
            deque.pop_back();
        }
        deque.push_back((ki, i));
    }
    println!("{sum}");
}
