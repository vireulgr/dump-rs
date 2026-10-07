use trpl::Html;
use std::process;

// async block translates into instance of anonymous data type that implements Future trait
// async function compiles into non-async function that returns async block wrapped into Future
// 
// fn page_title(url: &str) -> impl Future<Output = (&str, Option<String>)> {
//     async move { ... } // <- this block returns (&str, Option<String>)
// }
async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url)
        .await
        .text()
        .await;     // each await point represents a place where control is handed
                    // back to runtime. And here a structure representing async
                    // block state is created by runtime
                    //


    //(url, Some(text))
    let maybe_title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, maybe_title)
}

// await for futures only allowed in async functions or blocks
// top-level main function cannot be async, so we must wrap async code in trpl::block_on
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("two urls are required");
        process::exit(0);
    }
    let future_1 = page_title(&args[1]);
    let future_2 = page_title(&args[2]);

    // async runtime starts inside block_on
    trpl::block_on(async {

        let (url, maybe_title) = match trpl::select(future_1, future_2).await {
            trpl::Either::Left(left) => left,
            trpl::Either::Right(right) => right,
        };

        println!("URL {url} returned first!");

        match maybe_title {
            Some(title) => println!("Title {title}"),
            None => println!("Page has NO title"),
        }
    });
}
