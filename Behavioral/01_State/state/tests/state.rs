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

  post.review_content(false);
  post.review_content(true);
    
  let content_addition = String::from("\nSome more additions");
  post.add_content(content_addition);
  post.view_content();

  post.review_content(true);

  let content_addition_again = String::from("\nSome more additions");
  post.add_content(content_addition_again);
  post.review_content(false);
  post.view_content();

  // TODO: Check and add more tests if necessary                    
}