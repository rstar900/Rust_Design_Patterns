
#[derive(Debug, PartialEq)]
pub struct Post {
    // TODO: Add members
}

struct Draft {
    // TODO: Add members
}

struct InReview {
   // TODO: Add members
}

struct Published {
    // TODO: Add members
}

trait PostState {
    // TODO: Add state related functions
}

impl PostState for Draft {
    // TODO: Implement state related functions
}

impl PostState for InReview {
    // TODO: Implement state related functions
}


impl PostState for Published {
    // TODO: Implement state related functions
}

impl Post {
   pub fn new() -> Self {
        Post {
            // TODO: Initialize with first state
        }
    }

    // TODO: Implement state related functions
}