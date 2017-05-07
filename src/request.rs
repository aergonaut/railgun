//! Contains request and data guards for use by handlers.

use crypto;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use rocket::{Data, Outcome};
use rocket::data;
use rocket::data::FromData;
use rocket::http::Status;
use rocket::request;
use rocket::request::Request;
use rocket::request::FromRequest;
use std;
use std::io::prelude::*;

const X_GITHUB_EVENT: &'static str = "X-GitHub-Event";

const PULL_REQUEST_EVENT: &'static str = "pull_request";
const ISSUE_COMMENT_EVENT: &'static str = "issue_comment";
const STATUS_EVENT: &'static str = "status";

/// Describes the GitHub webhook event that triggered this request.
#[derive(Clone, Debug, PartialEq)]
pub enum GitHubEvent {
    PullRequest,
    IssueComment,
    Status
}

impl<'r, 'a> FromRequest<'r, 'a> for GitHubEvent {
    type Error = ();

    fn from_request(request: &'r Request<'a>) -> request::Outcome<GitHubEvent, ()> {
        let keys = request.headers().get(X_GITHUB_EVENT).collect::<Vec<_>>();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let event = match keys[0] {
            PULL_REQUEST_EVENT => GitHubEvent::PullRequest,
            ISSUE_COMMENT_EVENT => GitHubEvent::IssueComment,
            STATUS_EVENT => GitHubEvent::Status,
            _ => { return Outcome::Failure((Status::BadRequest, ())); }
        };

        Outcome::Success(event)
    }
}

const X_HUB_SIGNATURE: &'static str = "X-Hub-Signature";

/// Data guard that validates integrity of the request body by comparing with a
/// signature.
#[derive(Debug, PartialEq)]
pub struct SignedPayload(pub String);

impl FromData for SignedPayload {
    type Error = ();

    fn from_data(request: &Request, data: Data) -> data::Outcome<SignedPayload, ()> {
        let keys = request.headers().get(X_HUB_SIGNATURE).collect::<Vec<_>>();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let signature = keys[0];

        let mut body = String::new();
        if let Err(_) = data.open().read_to_string(&mut body) {
            return Outcome::Failure((Status::InternalServerError, ()));
        }

        let secret = match std::env::var("GITHUB_WEBHOOK_SECRET") {
            Ok(s) => s,
            Err(_) => { return Outcome::Failure((Status::InternalServerError, ())); }
        };

        if !is_valid_signature(&signature, &body, &secret) {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        Outcome::Success(SignedPayload(body))
    }
}

fn is_valid_signature(signature: &str, body: &str, secret: &str) -> bool {
    let digest = Sha1::new();
    let mut hmac = Hmac::new(digest, secret.as_bytes());
    hmac.input(body.as_bytes());
    let expected_signature = hmac.result();

    let parts = signature.splitn(2, '=').collect::<Vec<_>>();
    let code = parts[1];

    crypto::util::fixed_time_eq(bytes_to_hex(expected_signature.code()).as_bytes(), code.as_bytes())
}

const CHARS: &'static [u8] = b"0123456789abcdef";

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut v = Vec::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        v.push(CHARS[(byte >> 4) as usize]);
        v.push(CHARS[(byte & 0xf) as usize]);
    }

    unsafe {
        String::from_utf8_unchecked(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_signature() {
        let signature = "sha1=51633b546c869c7de65ce2f44d0c5eb49c0e5101";
        let body = "hello_world";
        let secret = "SUPERS3CR3T";

        assert!(is_valid_signature(signature, body, secret));
    }
}
