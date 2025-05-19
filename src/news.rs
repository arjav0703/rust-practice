// Traits define what behavior a type must implement.

pub trait Formatter {
    fn format(&self) -> String;
}

pub struct NewsArticle {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Formatter for NewsArticle {
    fn format(&self)  -> String {
        format!("{} {} says that {}", self.title, self.author, self.content)
    }
}

