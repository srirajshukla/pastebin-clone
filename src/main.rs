#![feature(proc_macro_hygiene, decl_macro)] // for using the nightly version

#[macro_use] extern crate rocket;

fn main() {
    rocket::ignite().launch();
}
