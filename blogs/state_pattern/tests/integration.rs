use state_pattern::Post;

#[test]
fn returns_empty_in_draft_state_without_text_added() {
    let post = Post::new();

    assert_eq!("", post.content());
}

#[test]
fn returns_empty_in_draft_state_with_text_added() {
    let mut post = Post::new();
    post.add_text("not empty");

    assert_eq!("", post.content());
}

#[test]
fn returns_empty_in_pending_review_state() {
    let mut post = Post::new();

    post.add_text("not empty");
    post.request_review();

    assert_eq!("", post.content());
}

#[test]
fn returns_content_in_published_state() {
    let mut post = Post::new();

    post.add_text("not empty");
    post.request_review();
    post.approve();

    assert_eq!("not empty", post.content());
}
