// Note: Do not communicate by sharing memory; instead, share memory by communicating
// Use channels if possible

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        // Cloning reference to counter to every thread
        let counter = Arc::clone(&counter);
        // Handle needs for join (waiting all to be finished)
        let handle = thread::spawn(move || {
            // Changing under mutex, it will be freed after going out of scope
            let mut num = counter.lock().unwrap();

            // Adding to counter an number of thread
            *num += i;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
