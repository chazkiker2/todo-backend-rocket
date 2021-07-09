#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World"
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![index])
        .attach(cors_fairing())
        .register(catchers![not_found])
}
