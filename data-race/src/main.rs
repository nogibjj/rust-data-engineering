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
