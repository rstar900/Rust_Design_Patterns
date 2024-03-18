use state::*;

#[test]
fn post_test() {
  // test the construction
    let mut post: Post = Post::new();
    assert_eq!(post, Post{post_content: String::from(""), 
                          post_state: Some(Box::new(Draft{}))
                        });

  // view initial content
  post.view_content();

  // Add content
  let mut content = String::from("Hello from published post");
  post.add_content(content);  

  // TODO: More tests                    
}