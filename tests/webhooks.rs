extern crate dotenv;
extern crate railgun;
extern crate rocket;

use rocket::http::{Method, Status};
use rocket::testing::MockRequest;

#[test]
fn test_webhook_endpoint() {
    dotenv::dotenv().ok();

    let app = railgun::app();

    let mut request = MockRequest::new(Method::Post, "/webhooks");
    let response = request.dispatch_with(&app);

    assert_eq!(Status::Ok, response.status());
}
