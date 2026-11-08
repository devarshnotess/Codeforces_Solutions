use std::io::{self, BufRead};
use std::cmp::Ordering;
 
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
 
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
 
    for _ in 0..t {
        let nm: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (n, m) = (nm[0], nm[1]);
 
        let mut cows: Vec<Vec<i32>> = Vec::with_capacity(n);
 
        for _ in 0..n {
            let mut row: Vec<i32> = lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            row.sort();
            cows.push(row);
        }
 
        let mut smallest_cards: Vec<(i32, usize)> = cows
            .iter()
            .enumerate()
            .map(|(i, hand)| (hand[0], i))
            .collect();
 
        smallest_cards.sort_by(|a, b| a.0.cmp(&b.0));
 
        let permutation: Vec<usize> = smallest_cards.iter().map(|&(_, idx)| idx).collect();
 
        let mut hands = cows.clone();
        let mut valid = true;
        let mut top = -1;
 
        for _ in 0..m {
            for &cow_id in &permutation {
                let mut found = false;
 
                for k in 0..hands[cow_id].len() {
                    if hands[cow_id][k] > top {
                        top = hands[cow_id][k];
                        hands[cow_id].remove(k);
                        found = true;
                        break;
                    }
                }
 
                if !found {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
        }
 
        if valid {
            for &idx in &permutation {
                print!("{} ", idx + 1);
            }
            println!();
        } else {
            println!("-1");
        }
    }
}