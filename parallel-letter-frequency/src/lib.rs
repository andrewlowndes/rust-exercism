use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let inputs_str = input.iter().map(|str| str.to_string()).collect::<Vec<_>>();
    let inputs = Arc::new(Mutex::new(inputs_str));
    let mut results = HashMap::new();
    let mut handles = vec![];

    for _ in 0..worker_count {
        let inputs_rc = inputs.clone();

        let handle = thread::spawn(move || {
            let mut chars_map = HashMap::new();

            while let Some(next_str) = inputs_rc.lock().unwrap().pop() {
                next_str
                    .chars()
                    .map(|char| char.to_ascii_lowercase())
                    .filter(|char| char.is_alphabetic())
                    .for_each(|char| {
                        chars_map
                            .entry(char)
                            .and_modify(|count| *count += 1)
                            .or_insert(1 as usize);
                    });
            }

            chars_map
        });

        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        
        for (key, value) in result {
            results
                .entry(key)
                .and_modify(|count| *count += value)
                .or_insert(value);
        }
    }

    results
}