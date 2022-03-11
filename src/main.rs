#[macro_use] extern crate rocket;

use rocket::response::content::Html;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/banned")]
fn banned() -> &'static str {
    "You have been banned from this site."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/banned", routes![banned])
}

