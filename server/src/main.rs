#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

// use std::sync::{Arc, Mutex};
use rocket::State;

struct Storage {
    sensor_data: std::vec::Vec::<std::string::String>,
    update_data: std::vec::Vec::<std::string::String>,
}

type StoragePtr = std::sync::Arc<std::sync::Mutex<Storage>>;

// TODO: check method names syntax
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

fn main() {
    rocket::ignite()
        .mount("/", routes![index, sensors, updates])
        .manage(make_async_storage())
        .launch();
}
