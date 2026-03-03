const MOD: u64 = 1e9 as u64 + 7;

fn power(a: u64, n: u64) -> u64 {
    match n {
        0 => 1,
        1 => a,
        _ => {
            let half = power(a, n / 2);
            let sq = (half * half) % MOD;
            if n % 2 == 1 { (sq * a) % MOD } else { sq }
        }
    }
}

fn inv(a: u64) -> u64 {
    power(a, MOD - 2)
}

fn comb(n: u64, k: u64, fact: &Vec<u64>) -> u64 {
    let n = n as usize;
    let k = k as usize;
    ((fact[n] * inv(fact[k]) % MOD) * inv(fact[n - k])) % MOD
}
