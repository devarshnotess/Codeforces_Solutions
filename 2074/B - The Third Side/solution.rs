use std::collections::BinaryHeap;
use std::io::{self, Write};
 
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
 
    let mut output = String::new();
 
    for _ in 0..t {
        // Read the number of elements in this test case
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
 
        // Read the array elements
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut a: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
 
        // Create a max-heap (using a BinaryHeap, which is a min-heap by default)
        let mut pq = BinaryHeap::new();
        for &num in &a {
            pq.push(num);
        }
 
        // Simulate the process until only one element is left
        while pq.len() > 1 {
            let a = pq.pop().unwrap(); // Get the largest element
            let b = pq.pop().unwrap(); // Get the second largest element
 
            let x = a + b - 1; // New value formed by a and b
 
            pq.push(x); // Insert the new value back into the heap
        }
 
        // The last remaining element is the result
        output.push_str(&format!("{}
", pq.pop().unwrap()));
    }
 
    io::stdout().write_all(output.as_bytes()).unwrap();
}