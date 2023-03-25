#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::env::current_dir;
use std::path::PathBuf;

use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;

use sensor_lab_backend::{div_start, format_row, sort_data, table_end, table_start};

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let static_dir = current_dir().unwrap().join("static");
    let file = static_dir.join(file);
    NamedFile::open(file).await.ok()
}

#[get("/")]
fn project<'a>() -> RawHtml<&'a str> {
    RawHtml(include_str!("../static/index.html"))
}

async fn poll_python() -> HashMap<String, (u32, u32)> {
    let mut data = HashMap::new();
    data.insert("test1".to_string(), (1, 1));
    data.insert("test2".to_string(), (2, 2));
    data.insert("test3".to_string(), (3, 3));
    data
}

#[get("/")]
async fn api() -> RawHtml<String> {
    let data = poll_python().await;
    let mut template = include_str!("../static/template.html").to_string();
    template = template.replace(
        r"<!--  {data}-->",
        div_start("heading".to_string()).as_str(),
    );
    template = template.replace(r"<!--  {data}-->", table_start().as_str());
    let values = data
        .values()
        .map(|(_order, value)| *value)
        .collect::<Vec<u32>>();
    let max = values.iter().max().unwrap();
    let data = sort_data(&data);
    for (key, _order, value) in data {
        let value = format!("{}", value);
        template = template.replace(
            r"<!--  {data}-->",
            format_row(key.clone(), value, max).as_str(),
        );
    }
    template = template.replace(r"<!--  {next_data}-->", table_end().as_str());
    RawHtml(template)
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![files])
        .mount("/project", routes![api])
}
