use ::oop::*;

fn main() {
    let mut article = Article::new();

    article.add_text("a quick brown fox jumps over a lazy dog");

    article.send_to_review();

    article.approve();

    let text = article.content();
    println!("article content: {}", text);
}
