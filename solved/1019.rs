// Problem: 책 페이지
// Contest: unknown_contest
// Judge: Baekjoon Online Judge
// URL: https://www.acmicpc.net/problem/1019
// Memory Limit: 128
// Time Limit: 2000
// Start: Wed 26 Mar 2025 03:11:28 PM KST

use std::io::stdin;

fn main() {
    let mut input_str = String::new();
    let _ = stdin().read_line(&mut input_str);
    let num: i32 = input_str.trim().parse().expect("Input not an integer");
    let result = get_digits(num);
    for cnt in result.iter().take(10) {
        print!("{} ", cnt);
    }
}

fn get_digits(num: i32) -> Vec<i64> {
    let mut res: Vec<i64> = Vec::new();
    for i in 0..10 {
        let digit_cnt = get_digit(num as i64, i);
        res.push(digit_cnt);
    }
    res
}

fn get_digit(num: i64, digit: usize) -> i64 {
    let dg: i64 = digit as i64;
    let mut sum: i64 = 0;
    let mut d: i64 = 1;
    while d <= num {
        let left = num / d;
        let curr = left % 10;
        let left = left / 10;
        let right = num % d;
        if dg != 0 {
            sum += left * d;
            match curr.cmp(&dg) {
                std::cmp::Ordering::Greater => {
                    sum += d;
                }
                std::cmp::Ordering::Equal => {
                    sum += right + 1;
                }
                _ => {}
            }
        } else {
            match curr {
                0 => {
                    sum += (left - 1) * d;
                    sum += right + 1;
                }
                _ => {
                    sum += left * d;
                }
            }
        }
        d *= 10;
    }
    sum
}
