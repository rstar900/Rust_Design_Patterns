use state::Post;

#[test]
fn post_contructs() {
    assert_eq!(Post::new(), Post{});
}