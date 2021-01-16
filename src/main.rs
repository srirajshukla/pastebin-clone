#![feature(proc_macro_hygiene, decl_macro)] // for using the nightly version
#[macro_use] extern crate rocket;

use rocket::Rocket;

#[get("/")]
fn index() -> &'static str{
    "
    USAGE

        POST /

            accepts raw data in the body of the requests and responds
            with a URL of a page containing the body's content.

        GET /<id>

            retrieves the content for the paste with id `<id>`
    "
}

fn rocket() -> Rocket{
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    rocket().launch();
}
