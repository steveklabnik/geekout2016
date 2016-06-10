use std::thread;
use std::sync::Arc;

fn main() {
    let xs = Arc::new(vec![1, 2, 3, 4, 5]);

    let mut handles = Vec::new();

    for _ in 0..3 {
        let xs = xs.clone();

        let handle = thread::spawn(move || {
            println!("{:?}", xs);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
