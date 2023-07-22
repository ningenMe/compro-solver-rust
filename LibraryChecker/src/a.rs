use proconio::input;

fn main() {
    input!{
        t: i32
    }
    for _ in 0..t {
        input!{
            a: i64,
            b: i64
        }
        println!("{}\n", a+b);            
    }
}
