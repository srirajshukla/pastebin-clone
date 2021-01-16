#![feature(proc_macro_hygiene, decl_macro)] // for using the nightly version
#[macro_use] extern crate rocket;
mod paste_id;

use std::path::Path;
use rocket::{Rocket, Data};
use paste_id::PasteId;

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

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String, std::io::Error> {
    let id = PasteId::new(3);
    let filename = format!("upload/{}", id);
    let url = format!("{host}/{id}\n", host="http://localhost:8000", id = id);

    // Writing the paste out to the file and return the URL.
    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

fn rocket() -> Rocket{
    rocket::ignite().mount("/", routes![index, upload])
}

fn main() {
    rocket().launch();
}
