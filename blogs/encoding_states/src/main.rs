use encoding_states::Post;

fn main() {
    // Create New Post and Add User Input
    let mut post = Post::new();
    post.add_text();

    // Post Review for Approval
    let post = post.request_review();
    let post = post.approve();

    // Display Post
    println!("\nNew Post:\n{}\n", post.content());
}
