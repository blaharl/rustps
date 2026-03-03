// Problem: 2-SAT - 4
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/11281
// Memory Limit: 256
// Time Limit: 1000
// Start: Thu 03 Apr 2025 07:29:59 PM KST

use std::cmp;

struct TarjanSCC {
    scc: Vec<usize>,
    t_in: Vec<usize>,
    stack: Vec<usize>,

    scc_cnt: usize,
    v_cnt: usize,
}

impl TarjanSCC {
    fn set_scc(&mut self, v: usize, adj: &Vec<Vec<usize>>) -> usize {
        self.t_in[v] = self.v_cnt;
        self.v_cnt += 1;
        let mut ret: usize = self.t_in[v];
        self.stack.push(v);

        for to in &adj[v] {
            let to = *to;
            if self.t_in[to] == usize::MAX {
                ret = cmp::min(ret, self.set_scc(to, adj));
            } else if self.scc[to] == usize::MAX {
                ret = cmp::min(ret, self.t_in[to]);
            }
        }

        if ret == self.t_in[v] {
            loop {
                let t = *self.stack.last().unwrap();
                self.stack.pop();
                self.scc[t] = self.scc_cnt;
                if t == v {
                    break;
                }
            }
            self.scc_cnt += 1;
        }
        ret
    }

    fn get_scc(&self) -> Vec<usize> {
        self.scc.clone()
    }

    fn init(&mut self, adj: &Vec<Vec<usize>>) {
        self.scc = vec![usize::MAX; adj.len()];
        self.t_in = self.scc.clone();
        for i in 0..adj.len() {
            if self.t_in[i] == usize::MAX {
                self.set_scc(i, adj);
            }
        }
    }

    fn new() -> TarjanSCC {
        TarjanSCC {
            scc: vec![],
            t_in: vec![],
            stack: vec![],

            scc_cnt: 0,
            v_cnt: 0,
        }
    }
}

struct TwoSat {
    n: usize,
    adj: Vec<Vec<usize>>,
}

impl TwoSat {
    fn build(n: usize) -> TwoSat {
        TwoSat {
            n,
            adj: vec![vec![]; 2 * n],
        }
    }

    fn set_adj(&mut self, x_: i32, y_: i32) {
        let x = x_.unsigned_abs() as usize - 1;
        let y = y_.unsigned_abs() as usize - 1;
        let nx = x + self.n;
        let ny = y + self.n;

        if x_ >= 0 && y_ >= 0 {
            self.adj[nx].push(y);
            self.adj[ny].push(x);
        } else if x_ >= 0 && y_ < 0 {
            self.adj[nx].push(ny);
            self.adj[y].push(x);
        } else if x_ < 0 && y_ >= 0 {
            self.adj[x].push(y);
            self.adj[ny].push(nx);
        } else {
            self.adj[x].push(ny);
            self.adj[y].push(nx);
        }
    }

    fn solve(&self) -> Vec<bool> {
        let mut tscc = TarjanSCC::new();
        tscc.init(&self.adj);
        let scc = tscc.get_scc();

        for i in 0..self.n {
            if scc[i] == scc[i + self.n] {
                return vec![];
            }
        }

        let nn = 2 * self.n;
        let mut visited = vec![false; self.n];
        let mut value = vec![false; self.n];
        let mut order = vec![(0_usize, 0_usize); nn];

        for i in 0..nn {
            order[i] = (scc[i], i);
        }

        order.sort_by(|a, b| b.cmp(a));

        for i in 0..nn {
            let v_ = order[i].1;
            let mut v = v_;
            let mut bv = true;

            if v_ >= self.n {
                v -= self.n;
                bv = false;
            }

            if visited[v] {
                continue;
            }

            visited[v] = true;
            value[v] = !bv;
        }

        value
    }
}

fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let n = next().parse().unwrap();
    let m = next().parse().unwrap();
    let mut tsat = TwoSat::build(n);
    for _ in 0..m {
        let x: i32 = next().parse().unwrap();
        let y: i32 = next().parse().unwrap();
        tsat.set_adj(x, y);
    }

    let val = tsat.solve();

    if val.is_empty() {
        println!("0");
        return;
    }

    writeln!(stdout, "1").unwrap();

    for v in val {
        if v {
            write!(stdout, "1 ").unwrap();
        } else {
            write!(stdout, "0 ").unwrap();
        }
    }

    print!("{stdout}");
}
