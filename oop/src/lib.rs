pub trait State {
    fn content<'a, 'b>(&'b self, _article: &'a Article) -> &'a str {
        ""
    }

    fn send_to_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

pub struct Draft { }
impl State for Draft {
    fn send_to_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Reviewed{})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}


pub struct Reviewed { }
impl State for Reviewed {
    fn send_to_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Approved{})
    }
}

pub struct Approved { }
impl State for Approved {
    fn send_to_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Reviewed{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a, 'b>(&'b self, article: &'a Article) -> &'a str {
        &article.text
    }
}


pub struct Article {
    pub state: Option<Box<dyn State>>,
    pub text: String,
}

impl Article {
    pub fn new() -> Self {
        Article { state: Some(Box::new(Draft{})), text: String::from("this is a test") }
    }

    pub fn content(&mut self) -> &str {
        match self.state.take() {
            Some(s) => s.content(self),
            None => "",
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.text.push_str(text); 
    }

    pub fn send_to_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.send_to_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}
