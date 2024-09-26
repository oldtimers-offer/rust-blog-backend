mod api;
mod auth;
mod model;
mod repo;
mod schema;

use crate::api::{create_post, create_user, get_posts};
use auth::DbConn;
use rocket_db_pools::Database;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::init())
        .mount("/", rocket::routes![create_post, get_posts, create_user])
}
