use std::io::{self, BufRead};
 
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
 
   
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
 
    for _ in 0..t {
        
        let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
 
        if (k - 1) % 3 == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}