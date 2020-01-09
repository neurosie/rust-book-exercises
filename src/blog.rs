/// Extending the blog post type from 17.03.

#[derive(Debug)]
pub struct Post {
    content: String,
}

impl Post {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self, approvals_needed: u32) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
            approvals_needed,
        }
    }
}

#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
    approvals: u32,
    approvals_needed: u32,
}

impl PendingReviewPost {
    pub fn approve(mut self) -> Result<Post, PendingReviewPost> {
        self.approvals += 1;
        if self.approvals == self.approvals_needed {
            Ok(Post {
                content: self.content,
            })
        } else {
            Err(self)
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_post() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review(1);

        let post = post.approve().unwrap();

        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn reject() {
        let post = Post::new();

        let post = post.request_review(1);

        let _post = post.reject();
    }

    #[test]
    fn two_reviews() {
        let mut post = Post::new();

        post.add_text("salad tiem");

        let post = post.request_review(2);

        let post = post.approve().unwrap_err();
        let post = post.approve().unwrap();

        assert_eq!("salad tiem", post.content());
    }
}
