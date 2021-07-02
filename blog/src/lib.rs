// Create the Post struct, storing its state as a trait object
// using the State trait. This allows Post to hold any type that
// implements the State trait in its state field.
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, s: &str) {
        self.content.push_str(s);
    }

    pub fn content(&self) -> &str {
        // Use as_ref to get the value inside the Option type. We want
        // to reference this value, not take ownership over it.
        self.state.as_ref().unwrap().content(self)
    }

    // Here, we call the request_review method on the state object.
    // All structs implementing the State trait will implement this
    // method. The idea is that a Draft struct will transition state
    // to PendingReview, but a PendingReview struct will just return
    // itself directly.
    pub fn request_review(&mut self) {
        // self.state.take() will move the Some value out of the state
        // field and replace it with None.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // Define a default implementation for content to avoid having
    // to define its implementation for each struct implementing
    // the State trait.
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    // Note that Box<Self>, not self, &self, or &mut self, is the type of
    // self here. This means the method is only valid when called on a Box
    // holding this type. Ownership belongs to Box<Self>, which allows us to
    // invalidate the old state and replace it with a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
