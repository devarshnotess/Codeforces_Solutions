use std::io::{self, BufRead};
 
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
 
    let t = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
 
    for _ in 0..t {
        let n = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
        let s = lines.next().unwrap().unwrap();
 
        let count_less = s.chars().filter(|&ch| ch == '<').count();
 
        let arr: Vec<usize> = (1..=n).collect();
 
        let mut result = Vec::with_capacity(n);
        let mut left = count_less as isize - 1;
        let mut right = count_less + 1;
 
        result.push(arr[count_less]);
 
        for ch in s.chars() {
            if ch == '<' {
                result.push(arr[left as usize]);
                left -= 1;
            } else {
                result.push(arr[right]);
                right += 1;
            }
        }
 
        let output = result.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", output);
    }
}