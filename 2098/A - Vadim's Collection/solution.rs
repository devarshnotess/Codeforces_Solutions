use std::io::{self, BufRead};
 
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
 
    let t = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
 
    for _ in 0..t {
        let s = lines.next().unwrap().unwrap();
        let mut count = [0; 10];
        for ch in s.chars() {
            count[ch.to_digit(10).unwrap() as usize] += 1;
        }
 
        let mut result = String::new();
        for i in 0..10 {
            let min_required = 9 - i;
            for d in min_required..=9 {
                if count[d] > 0 {
                    result.push(char::from_digit(d as u32, 10).unwrap());
                    count[d] -= 1;
                    break;
                }
            }
        }
 
        println!("{}", result);
    }
}