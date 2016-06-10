use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let xs = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));

    let mut handles = Vec::new();

    for i in 0..5 {
        let xs = xs.clone();

        let handle = thread::spawn(move || {
            let mut xs = xs.lock().unwrap();

            xs[i] += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", xs);
}
