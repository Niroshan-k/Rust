
fn fib (n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        let mut seq = vec![0, 1];
        
        for _ in 2..=n{
            let len = seq.len();
            let next = seq[len - 1] + seq[len - 2];
            seq.push(next);
        }
        //println!("{:?}", seq);
        seq[n as usize]
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
