#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json};

#[get("/")]
fn hello_json() -> Value {
    json!({
        "status": "204",
        "reason": "Success with no content"
    })
}

#[get("/employee/<_id>")]
fn get_employee(_id : i32) -> Value {
    json!({
        "Name": "Srinivas",
        "Title": "Rust Architect"
    })
}

#[catch(404)]
fn not_found() -> Value {
    json!({ "status": "Not found!"})
}


#[rocket::main]
async fn main() {
    let _= rocket::build()
        .mount("/", routes![hello_json,get_employee])
        .register("/", catchers![not_found])
        .launch()
        .await;
}