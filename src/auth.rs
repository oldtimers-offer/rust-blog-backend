use crate::model::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_db_pools::Connection;

// Basic authentication (placeholder for real JWT or session-based auth)
#[derive(rocket_db_pools::Database)]
#[database("postgres_db")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

pub struct AuthenticatedUser {
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Extract user from a basic header (e.g., Authorization: Bearer <token>), or use session-based logic.
        let username = req.headers().get_one("X-Username").unwrap_or("");
        let password = req.headers().get_one("X-Password").unwrap_or("");

        // Authenticate the user
        let mut db = req
            .guard::<Connection<DbConn>>()
            .await
            .expect("Can not connect to Postgres in request guard");

        // Now use Diesel's `first` method, which returns a Result
        let result = users::table
            .filter(users::username.eq(username))
            .filter(users::password.eq(password))
            .get_result(&mut db)
            .await;

        // Handle the result of the query
        match result {
            Ok(user) => Outcome::Success(AuthenticatedUser { user }),
            Err(_) => Outcome::Forward(Status::Unauthorized), // Return failure with Unauthorized status
        }
    }
}
