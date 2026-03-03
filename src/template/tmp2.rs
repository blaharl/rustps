#[allow(unused_imports)]
use std::cmp::{max, min};
use std::io::{BufWriter, Write, stdin, stdout};
#[allow(dead_code)]
const BITS: usize = 19;

#[allow(dead_code)]
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
#[allow(dead_code)]
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

#[allow(dead_code)]
fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<usize>();
    let q = scan.next::<usize>();
    let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();

    let mut closest = vec![n; BITS];
    let mut next = vec![vec![n; BITS]; n + 1];

    for (i, &ai) in a.iter().enumerate().rev() {
        let (zero_bits, one_bits): (Vec<usize>, Vec<usize>) =
            (0..BITS).partition(|b| (ai & (1usize << b)) == 0);

        for b in one_bits {
            next[i][b] = i;
            let j = closest[b];
            for &c in &zero_bits {
                // must borrow (not consume) zero_bits to reuse it!
                // inner loop's constant factor <= 1/4, so it may run 27 million times
                next[i][c] = min(next[i][c], next[j][c]);
            }
            closest[b] = i;
        }
    }

    for _ in 0..q {
        let x = scan.next::<usize>() - 1;
        let y = scan.next::<usize>() - 1;
        let success = (0..BITS)
            .filter(|b| (a[y] & (1usize << b)) != 0)
            .any(|b| next[x][b] <= y);
        if success {
            writeln!(out, "Shi").ok();
        } else {
            writeln!(out, "Fou").ok();
        }
    }
}
