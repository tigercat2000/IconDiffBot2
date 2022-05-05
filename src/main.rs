#![allow(non_snake_case)]

use dmm_tools::dmi::IconFile;
use rocket::{get, launch, routes};

#[get("/")]
async fn index() -> &'static str {
    "IDB says hello!"
}

#[launch]
async fn rocket() -> _ {
    let rocket = rocket::build();

    rocket.mount("/", routes![index])
}
