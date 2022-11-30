#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json};

#[get("/")]
fn hello_json() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[rocket::main]
async fn main() {
    let _= rocket::build()
        .mount("/", routes![hello_json])
        .launch()
        .await;
}