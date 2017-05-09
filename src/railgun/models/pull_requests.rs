use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use errors::*;
use schema::pull_requests;

#[derive(Debug, PartialEq, Queryable, Serialize)]
pub struct PullRequest {
    // pub id: Uuid,
    pub id: i32,
    pub repository: String,
    pub number: String,
    pub head_sha: String,
    pub status: String
}

#[derive(Insertable)]
#[table_name="pull_requests"]
pub struct PullRequestParams<'a> {
    pub repository: &'a str,
    pub number: &'a str,
    pub head_sha: &'a str,
    pub status: &'a str
}

/// Create a new Pull Request
pub fn create(params: &PullRequestParams, conn: &PgConnection) -> Result<PullRequest> {
    use schema::pull_requests;

    diesel::insert(params).into(pull_requests::table)
        .get_result(conn)
        .chain_err(|| "Error creating pull request")
}
