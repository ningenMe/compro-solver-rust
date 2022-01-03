use proconio::input;
use std::cmp::min;

fn putYn(flg : bool) {
    println!("{}", if flg {"Yes"} else {"No"}); 
}
fn getRev(s: String) -> Vec<char> {
    return s.chars().rev().collect::<Vec<char>>();
}

fn main() {
    input! {
        mut s1: String,
        mut s2: String,
    }
    let t1 = getRev(s1);
    let t2 = getRev(s2);
    let n = min(t1.len(),t2.len()) as usize;


    let mut flg = true;
    for i in 0_usize..n {
        let x = t1[i].to_digit(10).unwrap() + t2[i].to_digit(10).unwrap();
        if x >= 10 {
            flg = false;
        }
    }
    println!("{}",if flg {"Easy"} else {"Hard"});
}
