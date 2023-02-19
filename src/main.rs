#[macro_use]
extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello_name_age(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![hello, hello_name_age])
}
