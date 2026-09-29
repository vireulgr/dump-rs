use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    // references in thread function may outlive main thread lifetime
    // so values, captured by thread functions should be owned by thread function
    // moving values is one way to achieve this
    let handle = thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
        // val cannot be used after being sent
    });
    // tx cannot be used, it has been moved to thread
    let handle2 = thread::spawn(move || {
        let to_send = vec![
            String::from("this"),
            String::from("is"),
            String::from("a"),
            String::from("test"),
        ];
        for item in to_send {
            tx2.send(item).unwrap();
        }
    });
    // tx2 cannot be used, it has been moved to thread
    
    let text = rx.recv().unwrap();
    println!("Received: {}", text);

    // in this use thread will sleep until it receives message from tx
    for data in rx {
        println!("Received (2): {}", data);
    }

    // join will ensure that thread terminates and then main thread will continue execution
    handle.join().unwrap();
    handle2.join().unwrap();
}
