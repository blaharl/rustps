fn solve(stdin: &str) {
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let t = next().parse().unwrap();
    for _ in 0..t {
        let a: i32 = next().parse().unwrap();
        let b: i32 = next().parse().unwrap();
        println!("{}", a + b);
    }
}

fn main() {
    use std::io::*;
    solve(&read_to_string(stdin()).unwrap());
    STDOUT.with(|refcell| std::io::Write::flush(&mut *refcell.borrow_mut()).unwrap());
}

thread_local! {
    static STDOUT: std::cell::RefCell<std::io::BufWriter<std::io::StdoutLock<'static>>> = std::cell::RefCell::new(std::io::BufWriter::with_capacity(1 << 17, std::io::stdout().lock()));
}

#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {
        STDOUT.with(|refcell| {
            use std::io::*;
            writeln!(refcell.borrow_mut(), $($t)*).unwrap();
        });
    };
}
#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {
        STDOUT.with(|refcell| {
            use std::io::*;
            write!(refcell.borrow_mut(), $($t)*).unwrap();
        });
    };
}
