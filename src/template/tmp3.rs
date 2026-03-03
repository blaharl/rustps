#[allow(dead_code)]
fn main() {
    use std::fmt::Write;
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdout = String::new();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let t = next().parse().unwrap();
    for _ in 0..t {
        let a: i32 = next().parse().unwrap();
        let b: i32 = next().parse().unwrap();
        writeln!(stdout, "{}", a + b).unwrap();
    }
    print!("{stdout}");
}
