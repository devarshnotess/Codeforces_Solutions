use std::io::{self, BufRead, Write};
 
fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lines = stdin.lock().lines();
 
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let nm = lines.next().unwrap().unwrap();
        let mut iter = nm.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();
 
        let mut min_row = n + 1;
        let mut max_row = 0;
        let mut min_col = m + 1;
        let mut max_col = 0;
 
        for i in 1..=n {
            let row = lines.next().unwrap().unwrap();
            for (j, ch) in row.chars().enumerate() {
                if ch == '#' {
                    let col = j + 1;
                    min_row = min_row.min(i);
                    max_row = max_row.max(i);
                    min_col = min_col.min(col);
                    max_col = max_col.max(col);
                }
            }
        }
 
        let center_row = (min_row + max_row) / 2;
        let center_col = (min_col + max_col) / 2;
        writeln!(stdout, "{} {}", center_row, center_col).unwrap();
    }
}