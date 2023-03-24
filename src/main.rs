#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use std::env::current_dir;
use std::path::PathBuf;

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let static_dir = current_dir().unwrap().join("static");
    let file = static_dir.join(file);
    NamedFile::open(file).await.ok()
}

#[get("/")]
async fn project<'a>() -> RawHtml<&'a str> {
    RawHtml(include_str!("../static/index.html"))
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![files])
        .mount("/project", routes![project])
}
