use std::io;
 
fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
 
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
 
    
        let max_f = if n % 2 == 0 {
            n * n / 2
        } else {
            (n * n - 1) / 2
        };
        println!("{}", max_f / 2 + 1);
    }
}