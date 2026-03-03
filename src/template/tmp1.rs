#[allow(unused_imports)]
use std::cmp::{max, min};
use std::io::{BufWriter, Write, stdin, stdout};
#[allow(dead_code)]
const INF: i64 = 0x3f3f3f3f;

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
fn cross((x0, y0): (i64, i64), (x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0)
}

#[allow(dead_code)]
fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next();
    let p: Vec<_> = (0..n).map(|_| (scan.next(), scan.next())).collect();
    let s = format!("R{}R", scan.next::<String>());

    let mut taken = vec![false; n];
    let mut last = (INF, INF);

    for turn in s.chars() {
        let sgn = if turn == 'L' { 1 } else { -1 };
        let i = (0..n)
            .filter(|&i| !taken[i])
            .max_by(|&i, &j| (sgn * cross(last, p[i], p[j])).cmp(&0))
            .unwrap();
        taken[i] = true;
        last = p[i];
        write!(out, "{} ", i + 1).ok();
    }
    writeln!(out, "").ok();
}
// Similar ICPC problem: https://icpcarchive.ecs.baylor.edu/index.php?option=com_onlinejudge&Itemid=8&category=759&page=show_problem&problem=5721
