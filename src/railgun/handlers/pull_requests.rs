use db;
use diesel::prelude::*;
use errors::*;
use models::PullRequest;
use rocket::State;
use rocket_contrib::JSON;

#[get("/pull_requests")]
pub fn index(db_conn: State<db::ConnectionPool>) -> Result<JSON<Vec<PullRequest>>> {
    use schema::pull_requests::dsl::*;

    let conn = db_conn.get().chain_err(|| "Could not establish database connection")?;
    let results = pull_requests.limit(100).load::<PullRequest>(&*conn).chain_err(|| "Could not query pull database")?;
    Ok(JSON(results))
}

#[cfg(test)]
mod tests {
    use db;
    use dotenv;
    use rocket;
    use rocket::http::{Method, Status};
    use rocket::testing::MockRequest;

    #[test]
    fn test_index() {
        dotenv::dotenv().ok();

        let app = rocket::ignite()
            .manage(db::establish_connection_pool())
            .mount("/", routes![super::index]);

        let mut request = MockRequest::new(Method::Get, "/pull_requests");
        let response = request.dispatch_with(&app);

        assert_eq!(Status::Ok, response.status());
    }
}
