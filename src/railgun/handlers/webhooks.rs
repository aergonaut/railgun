use db;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use errors::*;
use models;
use models::PullRequestParams;
use request::{GitHubEvent, SignedPayload};
use rocket::State;
use serde_json as json;

#[post("/webhook", data = "<payload>")]
pub fn receive(event: Option<GitHubEvent>, payload: SignedPayload, db_conn: State<db::ConnectionPool>) -> Result<()> {
    let data = json::from_str::<json::Value>(&payload.0).chain_err(|| "Could not decode request body")?;
    let conn = db_conn.get().chain_err(|| "Could not establish database connection")?;
    match event {
        Some(GitHubEvent::PullRequest) => pull_request_event(&data, &*conn),
        _ => unimplemented!()
    }
}

macro_rules! otry {
    (
        $e:expr, $err:expr
    ) => (
        {
            match $e {
                Some(ret) => ret,
                None => bail!($err)
            }
        }
    )
}

fn pull_request_event(data: &json::Value, db_conn: &PgConnection) -> Result<()> {
    match data["action"].as_str() {
        Some("opened") => {
            let repo = otry!(data["repository"].as_str(), "No repository");
            let num = otry!(data["number"].as_i64(), "No number");
            let sha = otry!(data["head"]["sha"].as_str(), "No SHA");
            let new_pr = PullRequestParams {
                repository: repo,
                number: &num.to_string(),
                head_sha: sha,
                status: "opened"
            };
            let _ = models::pull_requests::create(&new_pr, db_conn)?;
            Ok(())
        },
        _ => bail!("Can't handle action")
    }
}

#[cfg(test)]
mod tests {
    use dotenv;
    use super::*;

    #[test]
    fn test_pull_request_event_with_missing_data_returns_error() {
        dotenv::dotenv().ok();

        let connection = db::establish_connection();
        let data = json!({
            "action": "opened",
            "repository": "octcat/spoon-knife",
            "head": {
                "sha": "abcdef1234567890"
            }
        });

        let result = pull_request_event(&data, &connection);

        match result {
            Err(Error(ErrorKind::Msg(m), _)) => assert_eq!(m, "No number"),
            _ => unreachable!()
        }
    }

    #[test]
    fn test_pull_request_event_creates_record() {
        use schema::pull_requests::dsl::*;

        dotenv::dotenv().ok();

        let connection = db::establish_connection();
        connection.begin_test_transaction().unwrap();

        let data = json!({
            "action": "opened",
            "repository": "octocat/spoon-knife",
            "number": 42,
            "head": {
                "sha": "abcdef1234567890"
            }
        });

        let _ = pull_request_event(&data, &connection).unwrap();

        let count = pull_requests
            .filter(repository.eq("octocat/spoon-knife"))
            .count()
            .get_result(&connection);

        assert_eq!(Ok(1), count);
    }
}
