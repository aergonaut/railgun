// use uuid::Uuid;

#[derive(Debug, PartialEq, Queryable, Serialize)]
pub struct PullRequest {
    // pub id: Uuid,
    pub id: i32,
    pub repository: String,
    pub number: String,
    pub head_sha: String,
    pub status: String
}
