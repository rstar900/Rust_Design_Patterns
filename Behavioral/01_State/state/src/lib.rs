use core::fmt::Debug;
use std::fmt::Formatter;
use dyn_partial_eq::*; // to derive PartialEq on trait objects


#[derive(Debug, PartialEq)]
pub struct Post {
    // Contains the content of the post and 
    // the pointer to the Object implementing PostState trait allocated on heap
    pub post_content: String,
    pub post_state: Option<Box<dyn PostState>>
}

impl Post {
    // Initially contain blank content and be on the Draft state
    pub fn new() -> Self {
         Post {
             post_content: String::from(""),
             post_state: Some(Box::new(Draft::new()))
         }
     }

    // State related functions

    pub fn view_content(&mut self) {
        // Move the content of the Option (state in this case) into another variable 's'
        // and make the post_state None, so that ownership of s is given to the function called
        // and not the post_state which needs to be owned by self
        if let Some(s) = self.post_state.take() {
            self.post_state = Some(s.view_content(self));
        }
    }

    pub fn add_content(&mut self, content: String) {
        // Move the content of the Option (state in this case) into another variable 's'
        // and make the post_state None, so that ownership of s is given to the function called
        // and not the post_state which needs to be owned by self
        if let Some(s) = self.post_state.take() {
            self.post_state = Some(s.add_content(self, content));
        }
    }

    pub fn review_content(&mut self, is_passing: bool) {
        // Move the content of the Option (state in this case) into another variable 's'
        // and make the post_state None, so that ownership of s is given to the function called
        // and not the post_state which needs to be owned by self
        if let Some(s) = self.post_state.take() {
            self.post_state = Some(s.review_content(is_passing));
        }
    }
 }

#[dyn_partial_eq]
pub trait PostState {
    // State related functions
    fn view_content(self: Box<Self>, context: &Post) -> Box<dyn PostState>;
    fn add_content(self: Box<Self>, context: &mut Post, content: String) -> Box<dyn PostState>;
    fn review_content(self: Box<Self>, is_passing: bool) -> Box<dyn PostState>; // No need of context as this is not viewing or modifying content 
}

// leaving this blank as we don't want to print box object
impl Debug for dyn PostState {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
}


#[derive(DynPartialEq, PartialEq)]
pub struct Draft {}

impl Draft {
    // Constructor with a message
    fn new() -> Self {
        println!("[Draft State:] Welcome!");
        Draft {}
    }
}

impl PostState for Draft {
    // Implement state related functions
    fn view_content(self: Box<Self>, context: &Post) -> Box<dyn PostState> {
        println!("[Draft State:] Cannot view post yet.");
        self
    }

    fn add_content(self: Box<Self>, context: &mut Post, content: String) -> Box<dyn PostState> {
        println!("[Draft State:] Added content, changing to InReview state...");
        context.post_content.push_str(content.as_str());
        Box::new(InReview::new())
    }

    fn review_content(self: Box<Self>, is_passing: bool) -> Box<dyn PostState> {
        println!("[Draft State:] Cannot review post yet.");
        self
    }
}

#[derive(DynPartialEq, PartialEq)]
pub struct InReview {}

impl InReview {
    // Constructor with a message
    fn new() -> Self {
        println!("[InReview State:] Welcome!");
        InReview {}
    }
}

impl PostState for InReview {

    // Implemention of state related functions

    fn view_content(self: Box<Self>, context: &Post) -> Box<dyn PostState> {
        println!("[InReview State:] Cannot view post yet.");
        self
    }

    fn add_content(self: Box<Self>, context: &mut Post, content: String) -> Box<dyn PostState> {
        println!("[InReview State:] Cannot edit post unless in Draft state.");
        self
    }

    fn review_content(self: Box<Self>, is_passing: bool) -> Box<dyn PostState> {
        if is_passing 
        {
            println!("[InReview State:] Review successful, changing to Published state...");
            Box::new(Published::new())
        }
        else
        {
            println!("[InReview State:] Review unsuccessful, changing back to draft state...");
            Box::new(Draft::new())
        }
    }            
}

#[derive(DynPartialEq, PartialEq)]
pub struct Published {}

impl Published {
    // Constructor with a message
    fn new() -> Self {
        println!("[Published State:] Welcome!");
        Published {}
    }
}

impl PostState for Published {

    // Implementation of state related functions

    fn view_content(self: Box<Self>, context: &Post) -> Box<dyn PostState> {
        println!("[Published State:] {}", context.post_content);
        self
    }

    fn add_content(self: Box<Self>, context: &mut Post, content: String) -> Box<dyn PostState> {
        println!("[Published State:] Cannot edit post unless in Draft state.");
        self
    }

    fn review_content(self: Box<Self>, is_passing: bool) -> Box<dyn PostState> {
        println!("[Published State:] Cannot review post after publishing.");
        self 
    }    
}