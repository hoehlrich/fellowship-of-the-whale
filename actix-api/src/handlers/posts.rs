use types::PostResponse;

use actix_web::web::{self, Json};
use actix_web::{error, Result};

use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};

use mysql::prelude::*;
use mysql::*;

use chrono::Utc;

#[derive(Debug, Display, Error)]
#[display(fmt = "PostError: {}", name)]
pub struct PostError {
    name: String,
}

impl error::ResponseError for PostError {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    id: String,
    title: String,
    author: String,
    categories: String,
    body: String,
}

pub async fn get_post(pool: web::Data<Pool>, id: web::Path<String>) -> Result<Json<PostResponse>, PostError> {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(PostError {
                name: e.to_string(),
            })
        }
    };

    match conn.query_map(
        format!("SELECT id, title, author, categories, body, timeCreated FROM posts WHERE id = {}", id),
        |(id, title, author, categories, body, time_created)| PostResponse {
            id,
            title,
            author,
            categories,
            body,
            time_created,
        },
    ) {
        Ok(selected_posts) => match selected_posts.iter().next() {
                Some(v) => Ok(Json(v.clone())),
                None => Err(PostError {
                    name: String::from(format!("Could not find post with id {}", id)),
                })
            }
        Err(e) => Err(PostError {
            name: e.to_string(),
        }),
    }
}

pub async fn get_posts(pool: web::Data<Pool>) -> Result<Json<Vec<PostResponse>>, PostError> {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(PostError {
                name: e.to_string(),
            })
        }
    };

    match conn.query_map(
        "SELECT id, title, author, categories, body, timeCreated from posts",
        |(id, title, author, categories, body, time_created)| PostResponse {
            id,
            title,
            author,
            categories,
            body,
            time_created,
        },
    ) {
        Ok(selected_posts) => Ok(Json(selected_posts)),
        Err(e) => Err(PostError {
            name: e.to_string(),
        }),
    }
}

pub async fn add_post(
    pool: web::Data<Pool>,
    post: web::Json<Post>
) -> Result<Json<()>, PostError> {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(PostError {
                name: e.to_string(),
            })
        }
    };

    let query_str = format!(
        r"INSERT INTO posts (id, title, author, categories, body, timeCreated)
        VALUES ('{}', '{}', '{}', '{}', '{}', '{}')",
        post.id,
        post.title,
        post.author,
        post.categories,
        post.body,
        Utc::now().format("%a %b %e %T %Y"),
    );

    match conn.query_drop(&query_str) {
        Ok(_) => Ok(Json(())),
        Err(e) => Err(PostError { name: e.to_string() }),
    }
}
