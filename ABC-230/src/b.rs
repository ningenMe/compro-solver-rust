use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: i32,
        ss: [String; n]
    }
    let mut map = HashMap::new();
    for s in ss {
        *map.entry(s).or_insert(0) += 1;
    }
    println!("{}",map.iter().max_by(|(_, a), (_, b)| a.cmp(b)).map(|(k, _)| k).unwrap());
}
