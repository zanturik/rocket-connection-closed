use rocket::form::Form;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    response::Redirect,
};

#[macro_use]
extern crate rocket;

struct SomeGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SomeGuard {
    type Error = std::convert::Infallible;

    async fn from_request(_request: &'r Request<'_>) -> Outcome<SomeGuard, Self::Error> {
        Outcome::Forward(Status::Unauthorized)
    }
}

#[derive(FromForm, Debug)]
pub struct SomeForm {
    pub field: String,
}

#[catch(401)]
fn unauthorized() -> Redirect {
    dbg!("unauthorized");
    Redirect::to(uri!(index))
}

#[get("/")]
async fn index<'a>() -> &'a str {
    "Hello, world!"
}

#[post("/unauthorized", data = "<_form>")]
fn post_test<'a>(_g: SomeGuard, _form: Form<SomeForm>) -> &'a str {
    "See no evil"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![unauthorized])
        .mount("/", routes![index, post_test])
}
