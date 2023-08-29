use crate::posts::model::Post;

#[derive(Debug, serde::Deserialize, sqlx::FromRow, serde::Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, serde::Deserialize, sqlx::FromRow, serde::Serialize, Clone)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, serde::Deserialize, sqlx::FromRow, serde::Serialize, Clone)]
pub struct UserPostResponse {
    pub name: String,
    pub username: String,
    pub email: String,

    pub posts: Vec<Post>,
}

impl UserPostResponse {
    pub fn response(user: UserResponse, post: Vec<Post>) -> UserPostResponse {
        return UserPostResponse {
            name: user.name,
            username: user.username,
            email: user.email,
            posts: post,
        };
    }
}

impl User {
    pub fn filter_user_record(self) -> UserResponse {
        UserResponse {
            id: self.id,
            email: self.email.to_owned(),
            name: self.name.to_owned(),
            username: self.username.to_owned(),
        }
    }
}
