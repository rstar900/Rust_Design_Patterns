use state::*;

#[test]
fn post_contructs() {
    let post: Post = Post::new();
    assert_eq!(post, Post{post_content: String::from(""), 
                          post_state: Box::new(Draft{context: None})
                        });
}