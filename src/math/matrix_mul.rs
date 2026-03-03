// Problem: 일하는 세포
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/17401
// Memory Limit: 512
// Time Limit: 1000
// Start: Wed 02 Apr 2025 09:09:27 PM KST

const DIV: i64 = 1_000_000_007;

fn multiply(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let row_a = a.len();
    let col_a = a[0].len();
    let row_b = b.len();
    let col_b = b[0].len();
    assert_eq!(col_a, row_b);

    let mut res = vec![vec![0_i64; col_b]; row_a];

    for i in 0..row_a {
        for j in 0..col_b {
            for k in 0..col_a {
                res[i][j] += a[i][k] * b[k][j];
                res[i][j] %= DIV;
            }
        }
    }

    res
}

fn get_i(sz: usize) -> Vec<Vec<i64>> {
    let mut res = vec![vec![0_i64; sz]; sz];
    for i in 0..sz {
        for j in 0..sz {
            if i == j {
                res[i][j] = 1;
            }
        }
    }

    res
}

fn pow(a: &Vec<Vec<i64>>, n: usize) -> Vec<Vec<i64>> {
    match n {
        0 => get_i(a.len()),
        1 => a.clone(),
        _ => {
            let mut half = pow(a, n / 2);
            half = multiply(&half, &half);
            if n % 2 == 1 {
                return multiply(&half, a);
            }
            half
        }
    }
}

fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let t: usize = next().parse().unwrap();
    let n: usize = next().parse().unwrap();
    let d: usize = next().parse().unwrap();

    let mut adj = vec![vec![vec![0_i64; n]; n]; t];
    for i in 0..t {
        let m: usize = next().parse().unwrap();
        for _ in 0..m {
            let a: usize = next().parse().unwrap();
            let a = a - 1;
            let b: usize = next().parse().unwrap();
            let b = b - 1;
            let c: i64 = next().parse().unwrap();
            adj[i][a][b] = c;
        }
        if i > 0 {
            adj[i] = multiply(&adj[i - 1], &adj[i]);
        }
    }
    let q = d / t;
    let r = d % t;
    let mut res = pow(&adj[t - 1], q);
    if r != 0 {
        res = multiply(&res, &adj[r - 1]);
    }

    for i in 0..n {
        for j in 0..n {
            write!(stdout, "{} ", res[i][j]).unwrap();
        }
        writeln!(stdout).unwrap();
    }
    print!("{stdout}");
}
