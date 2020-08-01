#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::VecDeque;
#[fastout]
fn main() {
    input! {
        mut s: Chars,
        q: usize,
    }
    let mut ans: VecDeque<char> = VecDeque::new();
    for i in s.iter() {
        ans.push_back(*i);
    }
    let mut t_1 = true;
    for _ in 0..q {
        input! {
            q: usize,
        }
        if q == 1 {
            t_1 = !t_1;
        } else {
            input! {
                f: usize,
                c: char,
            }
            let k: bool;
            if f == 1 {
                k = true;
            } else {
                k = false;
            }
            if t_1 == k {
                ans.push_front(c);
            } else {
                ans.push_back(c);
            }
        }
    }
    if t_1 == true {
        println!("{}", ans.iter().collect::<String>());
    } else {
        println!("{}", ans.iter().rev().collect::<String>());
    }
}
