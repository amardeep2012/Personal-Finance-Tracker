use rocket::{Request, Outcome};
use rocket::fairing::Guard;
use rocket::tokio::sync::Mutex;
use rocket::http::Status;

#[async_trait]
impl<'r> Guard<'r> for JwtAuth {
    fn from_request(request: &'r Request<'_>) -> rocket::tokio::sync::MutexResult<'r, Self> {
        let token = request.headers().get_one("Authorization");
        if token.is_some() && validate_jwt(token.unwrap()) {
            Outcome::Success(JwtAuth)
        } else {
            Outcome::Failure((Status::Unauthorized, "Invalid or missing token"))
        }
    }
}

pub struct JwtAuth;



