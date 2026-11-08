use std::io::{self, BufRead};
 
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
 
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
 
    for _ in 0..t {
        let k: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        if k % 2 == 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}