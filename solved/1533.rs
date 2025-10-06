// Problem: 길의 개수
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/1533
// Memory Limit: 128
// Time Limit: 2000
// Start: Sun 20 Apr 2025 04:18:22 PM KST

const DIV: u64 = 1_000_003;

fn multiply(a: &Vec<Vec<u64>>, b: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let row_a = a.len();
    let col_a = a[0].len();
    let row_b = b.len();
    let col_b = b[0].len();
    assert_eq!(col_a, row_b);

    let mut res = vec![vec![0_u64; col_b]; row_a];

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

fn get_i(sz: usize) -> Vec<Vec<u64>> {
    let mut res = vec![vec![0_u64; sz]; sz];
    for i in 0..sz {
        for j in 0..sz {
            if i == j {
                res[i][j] = 1;
            }
        }
    }

    res
}

fn pow(a: &Vec<Vec<u64>>, n: usize) -> Vec<Vec<u64>> {
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
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let n: usize = next().parse().unwrap();
    let mut adj = vec![vec![0_u64; 5 * n]; 5 * n];

    let s: usize = next().parse().unwrap();
    let e: usize = next().parse().unwrap();
    let s = s - 1;
    let e = e - 1;

    let t: usize = next().parse().unwrap();

    for i in 0..n {
        let line: String = next().parse().unwrap();
        for (j, dist) in line.chars().enumerate() {
            let dist = dist.to_digit(10).unwrap() as usize;
            if dist == 0 {
                continue;
            }
            adj[5 * i][5 * j + dist - 1] = 1;
        }
    }

    for i in 0..n {
        for j in 0..4 {
            adj[5 * i + j + 1][5 * i + j] = 1;
        }
    }

    let res = pow(&adj, t);
    println!("{}", res[s * 5][e * 5]);
}
