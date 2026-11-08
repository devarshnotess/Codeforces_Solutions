use std::io;
 
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
 
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<i64> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
 
        let (a, b, c) = (nums[0], nums[1], nums[2]);
        let total = a + b + c;
        let x = total / 3;
 
        if total % 3 != 0 || x < a || x < b || x > c {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}