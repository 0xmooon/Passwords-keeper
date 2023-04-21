use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, -----!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}