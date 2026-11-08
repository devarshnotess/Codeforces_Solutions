use std::io::{self, BufRead};
 
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
 
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
 
    for _ in 0..t {
    
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
 
     
        println!("{}", b - a);
    }
}