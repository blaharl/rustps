// Problem: Parcel
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/16287
// Memory Limit: 512
// Time Limit: 1000
// Start: Sun 30 Mar 2025 08:48:03 AM KST

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let w: i32 = next().parse().unwrap();
    let n: usize = next().parse().unwrap();
    let mut a: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        let ai: usize = next().parse().unwrap();
        a.push(ai);
    }
    a.sort();
    let mut two_sum: Vec<(usize, usize)> = vec![(0, 0); 400_001];
    for i in 0..n - 1 {
        for j in i + 1..n {
            let sum = a[i] + a[j];
            two_sum[sum] = (i + 1, j + 1);
            let left: i32 = w - sum as i32;
            if !(0..=400_000).contains(&left) {
                continue;
            }
            let (l, r) = two_sum[left as usize];
            if l != 0 {
                if l == i + 1 || l == j + 1 {
                    continue;
                }
                if r == i + 1 || r == j + 1 {
                    continue;
                }
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}
