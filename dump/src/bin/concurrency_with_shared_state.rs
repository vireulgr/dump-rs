use std::thread;
use std::sync::{Mutex, Arc};
//use std::time::Duration;

fn main() {
    let counter_ptr:  Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 1..=10 {
        let counter_clone = counter_ptr.clone();
        handles.push(
            thread::spawn(move|| {
                //                    Mutex => LockResult => MutexGuard
                let mut local_counter = counter_clone.lock().unwrap();
                *local_counter += 1;
            })
        );
    }

    //thread::sleep(Duration::from_millis(400));

    for handle in handles {
        handle.join().unwrap();
    }
    println!("count is {}", *counter_ptr.lock().unwrap());
}
