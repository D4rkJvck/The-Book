use state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today\n");
    post.request_review();
    post.approve();

    println!("New post:\n{}", post.content());
}
