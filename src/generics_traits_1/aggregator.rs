//pub mod aggregator {
    pub trait Summary {
        fn summarize_author(&self) -> String; 
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    // pub trait Summary {
    //     fn summarize(&self) -> String;
    // }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            String::from("wasya")
        }
        //fn summarize(&self) -> String {
        //    format!("{}, by {} ({})", self.headline, self.author, self.location)
        //}
    }

    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }

    impl Summary for SocialPost {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }
    // impl Summary for SocialPost {
    //     fn summarize(&self) -> String {
    //         format!("{}: {}", self.username, self.content)
    //     }
    // }
    // pub fn notify(item: &impl Summary) -> () {
    //     println!("Breaking news! {}", item.summarize());
    // }
    pub fn notify<T>(item: &T) -> () 
    where T: Summary 
    {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn returns_summarizable() -> impl Summary {
        SocialPost {
            username: String::from("qwe"),
            content: String::from("A quick brown fox jumps over a lazy dog"),
            reply: false,
            repost: false,
        }
    }
//}
