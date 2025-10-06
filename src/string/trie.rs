// Problem: 휴대폰 자판
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/5670
// Memory Limit: 192
// Time Limit: 1000
// Start: Sat 05 Apr 2025 02:19:33 PM KST

use std::collections::BTreeMap;

struct TrieNode {
    // TrieNode*
    children: BTreeMap<u8, TrieNode>,
    terminal: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: BTreeMap::new(),
            terminal: false,
        }
    }

    fn push(&mut self, str: &str) {
        if str.is_empty() {
            self.terminal = true;
            return;
        }

        let c = str.as_bytes().first().unwrap();
        let k = self.children.get(c);

        if k.is_none() {
            self.children.insert(*c, TrieNode::new());
        }
        self.children.get_mut(c).as_mut().unwrap().push(&str[1..]);
    }

    fn get_press_cnt(&self, cnt: u32, press_cnt: &mut Vec<u32>) {
        let mut branch: u32 = 0;

        if self.terminal {
            press_cnt.push(cnt);
            branch += 1;
        }

        for _ in &self.children {
            branch += 1;
        }

        for child in self.children.values() {
            if branch > 1 {
                child.get_press_cnt(cnt + 1, press_cnt);
            } else {
                child.get_press_cnt(cnt, press_cnt);
            }
        }
    }

    fn get_counts(&self, press_cnt: &mut Vec<u32>) {
        for child in self.children.values() {
            child.get_press_cnt(1, press_cnt);
        }
    }
}

fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next();
    while let Some(n_) = next() {
        let n = n_.parse().unwrap();
        let mut trie = TrieNode::new();
        for _ in 0..n {
            let s: String = next().unwrap().parse().unwrap();
            trie.push(&s);
        }
        let mut press_count = vec![0_u32; 0];
        trie.get_counts(&mut press_count);
        let mut sum = 0_u32;
        for i in &press_count {
            sum += i;
        }
        let avg = sum as f64 / press_count.len() as f64;
        writeln!(stdout, "{:.2}", avg).unwrap();
    }
    print!("{stdout}");
}
