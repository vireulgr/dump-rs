use std::thread;
use std::sync::{Mutex, Arc};
//use std::time::Duration;

// Rc does not implement Send trait; Arc does;
// Send trait allows ownership of values of types to be safely transferred from one thread to another
// Sync trait indicates that values can be safely accessed from other threads
// i.e. if &T implements Send then type T implements Sync
// RefCell and all types in std::cell does not implement Sync
// Rc does not implement Sync
// Mutes implement sync
// Manually implement sync and send is Unsafe!
fn main() {
    let counter_ptr = Arc::new(Mutex::new(0));
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
