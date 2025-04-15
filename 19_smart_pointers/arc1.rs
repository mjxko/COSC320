
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut join_handles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers
                .iter()
                .enumerate()
                .filter(|(i, _)| i % 8 == offset)
                .map(|(_, &val)| val)
                .sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }
    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}