use proconio::input;
use std::cmp;

fn yn(flg : bool) {
    println!("{}", if flg {"Yes"} else {"No"}); 
}

fn main() {
    input!{
        n: usize,
        mut A: [i32; n]
    }
    println!("{}", A.iter().map(|x| cmp::max(x-10,0)).sum::<i32>());
}
