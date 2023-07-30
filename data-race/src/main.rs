/*

// Mutex that protects the data vector, and then we spawn three threads 
//that each acquire a lock on the mutex and modify an element of the vector.

use std::sync::Mutex;
use std::thread;

fn main() {
    let data = Mutex::new(vec![1, 2, 3]);

    let handles: Vec<_> = (0..3).map(|i| {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data);
}

*/

use std::thread;

fn main() {
    let mut data = vec![1, 2, 3];

    for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
        thread::spawn(move || {
            data[i] += 1;
        });
    }

    // No data race can occur, this will not compile.
}
