use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}
