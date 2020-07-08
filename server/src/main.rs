#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use rocket::State;
use std::env;

pub mod models;
pub mod schema;

use models::*;


struct Storage {
    sensor_data: std::vec::Vec::<std::string::String>,
    update_data: std::vec::Vec::<std::string::String>,
}

type StoragePtr = std::sync::Arc<std::sync::Mutex<Storage>>;

fn make_async_storage() -> StoragePtr {
    std::sync::Arc::new(std::sync::Mutex::new(Storage{
        sensor_data: std::vec::Vec::<std::string::String>::new(),
        update_data: std::vec::Vec::<std::string::String>::new(),
    }))
}

#[get("/")]
fn index(storage: State<StoragePtr>) -> String {
    let storage = storage.lock();
    let data = storage.unwrap();
    return format!("Updates:\n{}\nSensors:\n{}", data.update_data.join("\n"), data.sensor_data.join("\n"));
}

#[post("/sensors", data = "<data>")]
fn sensors(data: String, storage: State<StoragePtr>) ->&'static str {
    (*storage.lock().unwrap()).sensor_data.push(data);
    return "Ok";
}

#[post("/updates", data = "<data>")]
fn updates(data: String, storage: State<StoragePtr>) ->&'static str {
    (*storage.lock().unwrap()).update_data.push(data);
    return "Ok";
}

#[get("/chart")]
fn graphic() -> &'static str {
    use gnuplot::{Figure, Caption, Color};

    let x = [0u32, 1, 2, 3, 4, 5];
    let y = [3u32, 4, 5, 1, 5, 2];
    let mut fg = Figure::new();
    fg.axes2d()
    .lines(&x, &y, &[Caption("Samle chart"), Color("black")]);
    fg.save_to_png("/Users/overtired/Desktop/graphic.png", 2000u32, 2000u32);
    fg.set_offset(100f32, 100f32);

    return "Drawn";
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


// fn test_db() {
//     use self::schema::sensor_data::dsl::*;
//     let connection = establish_connection();
//     let results = sensors_data
//         .limit(5)
//         .load::<SensorsData>(&connection)
//         .expect("Error loading data");

//     // println!("Displaying {} datas", results.len());
//     // for data in results {
//     //     println!("{}", data.timestamp);
//     //     println!("----------\n");
//     //     println!("{}", data.humidity);
//     // }
// }

fn main() {
    // test_db();
    rocket::ignite()
        .mount("/", routes![index, sensors, updates, graphic])
        .manage(make_async_storage())
        .launch();
}
