mod aggregator;

use std::fmt::Display;
use crate::aggregator::{SocialPost, Summary, notify};

struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

// if T is such that it implements Display and PartitionOrd traits
// then implement methods of Pair<T> for that T
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("largest number is {}", self.x);
        }
        else {
            println!("largest nubmer is {}", self.y);
        }
    }
}

fn main() {
    let var1 = SocialPost {
        username: String::from("vasya the fifth"),
        content: String::from("Trees are plenty. So are bushes"),
        repost: false,
        reply: false,
    };

    // let var2 = NewsArticle {
    //     headline: String::from("Scientist raped journalist!"),
    //     location: String::from("Netherlands"),
    //     author: String::from("Dmitry Gordon"),
    //     content: String::from("There is something on your mind"),
    // }

    println!("social post: {}", var1.summarize());

    notify(&var1);

    // ====================================================================
    
    let a: Pair<i32> = Pair::new(-12, 32);

    a.cmp_display();

}
