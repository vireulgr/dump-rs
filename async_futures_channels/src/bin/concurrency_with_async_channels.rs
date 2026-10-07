use std::time;


fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let tx2 = tx.clone();
        let fut_1 = async move { // move here needed to drop tx causing rx.recv() to return None
                                 // and finish while let loop
            let messages = vec![
                String::from("msg 1"),
                String::from("msg 2"),
                String::from("msg 3")
            ];

            for msg in messages {
                tx2.send(msg).unwrap();
                trpl::sleep(time::Duration::from_millis(500)).await;
            }
        };

        let fut_2 = async {
            while let Some(msg) = rx.recv().await {
                println!("{msg}");
            }
        };

        let fut_3 = async move {
            let msgs = vec![
                String::from("2nd sender msg 1"), 
                String::from("2nd sender msg 2"), 
                String::from("2nd sender msg 3"), 
                String::from("2nd sender msg 4"), 
            ];
            for item in msgs {
                tx.send(item).unwrap();
                trpl::sleep(time::Duration::from_millis(300)).await;
            }
        };

        // order of arguments is meaningful here
        // first future executed first and then yields to second
        trpl::join!(fut_2, fut_1, fut_3);
    });
}
