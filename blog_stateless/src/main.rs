use blog_stateless::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("D3 is a powerful JavaScript visualization library.");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!(
        "D3 is a powerful JavaScript visualization library.",
        post.content()
    );
}
