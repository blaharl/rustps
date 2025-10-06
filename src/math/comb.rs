// Problem: 이항 계수와 쿼리
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/13977
// Memory Limit: 512
// Time Limit: 1000
// Start: Sun 06 Apr 2025 10:23:23 PM KST

const DIV: u64 = 1_000_000_007;

struct Comb {
    fact: Vec<u32>,
}

impl Comb {
    fn set_fact(&mut self) {
        self.fact = vec![0; 4_000_001];
        self.fact[0] = 1;
        self.fact[1] = 1;
        for i in 2..=4_000_000 {
            self.fact[i] = {
                let x = self.fact[i - 1] as u64 * i as u64 % DIV;
                x as u32
            }
        }
    }

    fn pow(a: u64, n: u64) -> u64 {
        match n {
            0 => 1,
            1 => a,
            _ => {
                let mut half = Comb::pow(a, n / 2);
                half *= half;
                half %= DIV;
                if n % 2 == 1 { half * a % DIV } else { half }
            }
        }
    }

    fn inv(num: u64) -> u64 {
        Comb::pow(num, DIV - 2)
    }

    fn get(&self, n: usize, r: usize) -> u64 {
        self.fact[n] as u64 * Comb::inv(self.fact[r] as u64 * self.fact[n - r] as u64 % DIV) % DIV
    }
}

fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut c = Comb { fact: vec![] };
    c.set_fact();
    let m = next().parse().unwrap();
    for _ in 0..m {
        let n: usize = next().parse().unwrap();
        let k: usize = next().parse().unwrap();
        writeln!(stdout, "{}", c.get(n, k)).unwrap();
    }
    print!("{stdout}");
}
