use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let m = if n >= 42 {n+1} else {n};
    println!("AGC{:>03}",m);
}
