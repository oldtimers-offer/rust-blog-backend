use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::model::*;
use crate::schema::*;

pub struct BlogRepo;

impl BlogRepo {
    pub async fn list_posts(p: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Post>> {
        posts::table.limit(limit).get_results(p).await
    }

    pub async fn create_post(p: &mut AsyncPgConnection, new_post: NewPost2) -> QueryResult<Post> {
        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result::<Post>(p)
            .await
    }
}

pub struct UserRepo;

impl UserRepo {
    pub async fn create_user(c: &mut AsyncPgConnection, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(c)
            .await
    }
}
