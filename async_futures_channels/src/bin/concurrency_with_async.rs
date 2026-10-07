use std::time::Duration;


fn main() {
    trpl::block_on(async {
        // task can be bound to OS thread so Rust's async runtime cannot guarantee fairness in
        // multiple tasks execution (some tasks may gain an advantage or disadvantage over others)
        let join_handle = trpl::spawn_task(async {
            for i in 1..=10 {
                println!("count {i} from 1st counter");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        }); // as soon as outer context is dropped, task is dropped too, regardless of its inner
            // state. To wait task to finish one must explicitly call await on join handle that
            // returned from spawn_task

        for i in 1..=5 {
            println!("count {i} from 2nd counter");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        join_handle.await.unwrap(); // wait task to count to 10
    });
}
