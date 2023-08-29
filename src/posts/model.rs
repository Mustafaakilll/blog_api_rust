#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub date_published: chrono::NaiveDateTime,
    pub author_id: i32,
    pub category_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub category_id: i32,
}

impl NewPost {
    pub fn new(title: String, content: String, author_id: i32, category_id: i32) -> Self {
        Self {
            title,
            content,
            category_id,
        }
    }
}
