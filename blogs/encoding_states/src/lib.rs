use std::{io, thread, time::Duration};

pub struct Post {
    content: String,
}

impl Post {
    /// Create a new draft to be published.
    pub fn new() -> Draft {
        Draft {
            content: String::new(),
        }
    }

    /// Get access to the post content.
    pub fn content(&self) -> &str {
        &self.content
    }
}

///////////////////////////////////////////////////////////////////////

pub struct Review {
    content: String,
}

impl Review {
    /// Validate the post to be published.
    pub fn approve(self) -> Post {
        println!("\nApproved!");
        thread::sleep(Duration::from_secs(1));
        Post {
            content: self.content,
        }
    }
}

///////////////////////////////////////////////////////////////////////

pub struct Draft {
    content: String,
}

impl Draft {
    /// Prompt the user to add the new post content.
    pub fn add_text(&mut self) {
        let mut input = String::new();
        println!("Enter your post here:");
        io::stdin().read_line(&mut input).expect("Invalid Input");
        self.content.push_str(&input);
        println!("Added!");
        thread::sleep(Duration::from_secs(1));
    }

    /// Check the user's input validity.        //REVIEW: Add Checking Logic to Approve Content
    pub fn request_review(self) -> Review {
        println!("\nReviewing...");
        thread::sleep(Duration::from_secs(3));
        Review {
            content: self.content,
        }
    }
}
