use proconio::input;

fn main() {
    input! {
        s: String
    }
    let n = s.len() as usize;
    let t = "oxx".repeat(100000);
    let mut flg = false;
    for i in 0..(t.len()) {
        let j = i as usize;
        if j+n <= t.len() && &t[j..j+n] == s {
            flg = true
        }
    }
    println!("{}",if flg {"Yes"} else {"No"});
}
