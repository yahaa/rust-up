#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod other {
    #[get("/world")]
    pub fn world() -> &'static str {
        "Hello, world!"
    }
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, outside world!"
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, other::world, hello])
        .launch();
}
