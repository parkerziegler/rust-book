use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("ReasonML is a new dialect of OCaml developed at Facebook.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(
        "ReasonML is a new dialect of OCaml developed at Facebook.",
        post.content(),
    )
}
