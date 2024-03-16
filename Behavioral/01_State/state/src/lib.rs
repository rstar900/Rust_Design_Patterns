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

// leaving this blank as we don't want to print box object
impl Debug for dyn PostState {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
}


impl Post {
    // Initially contain blank content and be on the Draft state
    pub fn new() -> Self {
         Post {
             post_content: String::from(""),
             post_state: Box::new(Draft{})
         }
     }
 
     // TODO: Implement state related functions
 }

#[dyn_partial_eq]
pub trait PostState {
    // State related functions
    fn view_content(&self);
    fn add_content(&self);
    fn review_content(&self);
}

#[derive(DynPartialEq, PartialEq)]
pub struct Draft {
    // TODO: Add members
}

impl PostState for Draft {
    // Implement state related functions
    fn view_content(&self) {
        
    }
    fn add_content(&self) {

    }
    fn review_content(&self) {

    }
}

#[derive(DynPartialEq, PartialEq)]
pub struct InReview {
   // TODO: Add members
}

impl PostState for InReview {
    // TODO: Implement state related functions
    fn view_content(&self) {
        
    }
    fn add_content(&self) {

    }
    fn review_content(&self) {
        
    }    
}

#[derive(DynPartialEq, PartialEq)]
pub struct Published {
    // TODO: Add members
}

impl PostState for Published {
    // TODO: Implement state related functions
    fn view_content(&self) {
        
    }
    fn add_content(&self) {

    }
    fn review_content(&self) {
        
    }    
}
