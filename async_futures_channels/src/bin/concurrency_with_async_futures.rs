use std::time;

fn main() {
    trpl::block_on(async {
        let fut_1 = async {
            for i in 1..=10 {
                println!("counter 1 {i}");
                trpl::sleep(time::Duration::from_millis(300)).await;
            }
        };

        let fut_2 = async {
            for i in 1..=5 {
                println!("counter 2 {i}");
                trpl::sleep(time::Duration::from_millis(200)).await;
            }
        };

        trpl::join(fut_1, fut_2).await; // join function is fair, it switches equally between futures
    });
}
