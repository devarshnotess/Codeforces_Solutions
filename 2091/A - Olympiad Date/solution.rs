use std::io::{self, BufRead};
 
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
 
 
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
 
    
    let mut target_count = [0; 10];
    for ch in "01032025".chars() {
        let digit = ch.to_digit(10).unwrap() as usize;
        target_count[digit] += 1;
    }
 
    for _ in 0..t {
        
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
 
        
        let a: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
 
        let mut current_count = [0; 10];
        let mut answer = 0;
 
        for (i, &digit) in a.iter().enumerate() {
            current_count[digit] += 1;
 
            
            let ok = (0..10).all(|d| current_count[d] >= target_count[d]);
 
            if ok {
                answer = i + 1; 
                break;
            }
        }
 
        println!("{}", answer);
    }
}