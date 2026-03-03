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
