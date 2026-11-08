use std::io::{self, BufRead};
 
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
 
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
 
    for _ in 0..t {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() == 3 {
                let c1 = parts[0].chars().next().unwrap();
                let c2 = parts[1].chars().next().unwrap();
                let c3 = parts[2].chars().next().unwrap();
                println!("{}{}{}", c1, c2, c3);
            }
        }
    }
}