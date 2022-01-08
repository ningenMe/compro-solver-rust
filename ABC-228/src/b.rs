use proconio::input;

fn Yn(flg : bool) {
    println!("{}", if flg {"Yes"} else {"No"}); 
}

fn main() {
    input! {
        N: usize,
        mut X: usize,
        mut A: [usize; N]
    }
    X -= 1;
    for i in 0..N {
        A[i] -= 1;
    }
    let mut B = vec![N+1; N];
    B[X] = N;
    loop {
        X = A[X];
        if B[X] == N {
            break;
        }
        B[X] = N;
    }
    println!("{}", B.iter().filter(|&&x| x == N).count()); 
}
