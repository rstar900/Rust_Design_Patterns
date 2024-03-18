use core::fmt::Debug;
use std::fmt::Formatter;
use dyn_partial_eq::*; // to derive PartialEq on trait objects


#[derive(Debug, PartialEq)]
pub struct Post {
    // Contains the content of the post and 
    // the pointer to the Object implementing PostState trait allocated on heap
    pub post_content: String,
    pub post_state: Box<dyn PostState>
}

impl Post {
    // Initially contain blank content and be on the Draft state
    pub fn new() -> Self {
         Post {
             post_content: String::from(""),
             post_state: Box::new(Draft::new())
         }
     }

     // State related functions
    fn view_content(&self) {
        self.post_state.view_content();
    }

    fn add_content(&self, content: String) {
        self.post_state.add_content(content);
    }

    fn review_content(&self, is_passing: bool) {
        self.post_state.review_content(is_passing);
    }
 }

#[dyn_partial_eq]
pub trait PostState {
    // State related functions
    fn view_content(&self);
    fn add_content(&self, content: String);
    fn review_content(&self, is_passing: bool);
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
    fn view_content(&self) {
        
    }

    fn add_content(&self, content: String) {

    }

    fn review_content(&self, is_passing: bool) {

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
    // TODO: Implement state related functions
    fn view_content(&self) {
        
    }

    fn add_content(&self, content: String) {

    }

    fn review_content(&self, is_passing: bool) {

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
    // TODO: Implement state related functions
    fn view_content(&self) {
        
    }
    fn add_content(&self, content: String) {

    }

    fn review_content(&self, is_passing: bool) {

    }    
}