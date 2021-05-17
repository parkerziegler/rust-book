use std::fmt::Debug;
use std::fmt::Display;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    // Here we call the summarize_short method with the default impl. from the Summary trait.
    println!("1 new article: {}", article.summarize_short());

    notify(&article);
}

// Summary is an example of a trait in Rust.
// Traits define shared behavior across many concrete types in an abstract way.
pub trait Summary {
    fn summarize(&self) -> String;
    // summarize_short has a default implementation.
    // You _can_ implement this method if implementing the Summary trait,
    // but if you don't this default impl. will be used instead.
    fn summarize_short(&self) -> String {
        format!("Read more from: {}", self.summarize_author())
    }
    // Note that summarize_author, which requires an impl., can be called by
    // the default implementation of summarize_short!
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Here we implement the Summary trait on the NewsArticle struct.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {} by {}", self.headline, self.location, self.author)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// And similarly, the implementation of Summary for Tweet.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// We can define functions that take traits as parameters.
// The notify function below can only be called with an argument
// that implements the Summary trait.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax allows us to constrain the types a function
// accepts even more tightly. For example, the following function
// accepts two arguments that _must_ be the same type and _must_
// implement the Summary trait.
pub fn notify_same<T: Summary>(item: &T, item_2: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Breaking news again! {}", item_2.summarize());
}

// You can also specify multiple trait bounds using the + operator.
// This specifies that the argument must implement _both_ traits.
pub fn notify_multiple<T: Summary + Display>(item: &T) {}

// A common syntax for specifying multiple parameters with
// multiple trait bounds is the where keyword.
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    10
}

// You can specify a return type that implements traits.
// For example, the following function returns a type that
// implements Summary, but it doesn't specify _which_ concrete
// type that is.
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("parker_ziegler"),
        content: String::from("Introducing v0.1.0 of renature!"),
        reply: false,
        retweet: true,
    }
}

// Lastly, we can use trait bounds to conditionally implement
// methods on types that implement certain traits.
// For example, we can create a Pair struct that only implements
// a cmp_display method if it's acting on a generic type that
// implements Display and PartialOrd.
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
