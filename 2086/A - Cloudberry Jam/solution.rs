use std::io::{self, BufRead};
 
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
 
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
 
    for _ in 0..t {
        let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        println!("{}", 2 * n);
    }
}