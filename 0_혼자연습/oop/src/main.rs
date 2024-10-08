use oop::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate sushi");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate sushi", post.content());
}