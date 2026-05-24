use std::collections::HashMap;
use std::thread;


  

pub fn frequency(input:&[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut handles = Vec::new();
   


    // Convert to owned Strings
    let vec: Vec<String> = input.iter().map(|s| s.to_string()).collect();


    // Spawn one thread per string
    for s in vec {
        let s = s.clone();

        let handle = thread::spawn(move || {
            let mut local_map: HashMap<char, usize> = HashMap::new();

            for c in s.chars() {
                if c.is_alphabetic() {
                    let c = c.to_ascii_lowercase();
                    *local_map.entry(c).or_insert(0) += 1;
                }
            }

            local_map
        });

        handles.push(handle);
    }

    // Merge results
    let mut final_map: HashMap<char, usize> = HashMap::new();

    for handle in handles {
        let local_map = handle.join().unwrap();

        for (ch, count) in local_map {
            *final_map.entry(ch).or_insert(0) += count;
        }
    }

    final_map
}

    
