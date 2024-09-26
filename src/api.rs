use crate::auth::AuthenticatedUser;
use crate::auth::DbConn;
use crate::model::NewUser;
use crate::model::{NewPost, NewPost2};
use crate::repo::{BlogRepo, UserRepo};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use std::error::Error;

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

#[rocket::get("/posts")]
pub async fn get_posts(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    BlogRepo::list_posts(&mut db, 100)
        .await
        .map(|posts| json!(posts))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/posts", format = "json", data = "<new_post>")]
pub async fn create_post(
    mut db: Connection<DbConn>,
    new_post: Json<NewPost>,
    auth_user: AuthenticatedUser,
) -> Result<Value, Custom<Value>> {
    if auth_user.user.is_author {
        let post = NewPost2 {
            title: new_post.title.clone(),
            body: new_post.body.clone(),
            author_id: auth_user.user.id.clone(),
        };

        BlogRepo::create_post(&mut db, post)
            .await
            .map(|post| json!(post))
            .map_err(|e| server_error(e.into()))
    } else {
        Err(Custom(
            Status::Forbidden,
            json!("Error: You are not authorized to create posts"),
        ))
    }
}

#[rocket::post("/register", format = "json", data = "<new_user>")]
pub async fn create_user(
    mut db: Connection<DbConn>,
    new_user: Json<NewUser>,
) -> Result<Custom<Value>, Custom<Value>> {
    UserRepo::create_user(&mut db, new_user.into_inner())
        .await
        .map(|user| Custom(Status::Created, json!(user)))
        .map_err(|e| server_error(e.into()))
}
