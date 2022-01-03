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
        mut N: i64,
        mut W: i64,
    }
    let mut ab = Vec::new();
    for i in 0..N {
        input! {
            a: i64,
            b: i64,
        }   
        ab.push((a,b));
    }
    ab.sort_by(|(x,_),(y,_)|x.cmp(&y).reverse());
    let mut ans :i64 = 0;
    let mut w = W;
    for (a,b) in ab {
        let c = min(b, w);
        w -= c;
        ans += c * a;
    }
    println!("{}",ans);
}
