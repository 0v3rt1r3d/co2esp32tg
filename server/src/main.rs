
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate dotenv;
extern crate plotters;

use rocket::State;
use rocket_contrib::json::Json;
use std::env;
use std::str;

mod chart;
mod handlers;
mod storage;
mod tgapi;


struct BotToken {
    token: String
}

// Temporary replacements till charts are not drawn
type SensorsDataPtr = std::sync::Arc<std::sync::Mutex<Option::<storage::SensorsData>>>;
fn make_async_sensors_data() -> SensorsDataPtr {
    std::sync::Arc::new(std::sync::Mutex::new(None))
}

#[get("/")]
fn index(storage: State<storage::StoragePtr>) -> String {
    return handlers::handle_index(&*storage);
}

#[post("/sensors", data = "<data>", format="json")]
fn sensors(
    data: Json<storage::SensorsData>,
    last_sd: State<SensorsDataPtr>,
    storage: State<storage::StoragePtr>
) ->&'static str {
    (*storage.lock().unwrap()).save_sensors(&data);

    // Temprorary
    let mut locked = last_sd.lock().unwrap();
    locked.replace(data.clone());

    return "Ok";
}

#[post("/updates", data = "<update>", format="json")]
fn updates(
    update: Json<tgapi::Update>,
    last_sd: State<SensorsDataPtr>,
    storage: State<storage::StoragePtr>,
    token: State<BotToken>
) ->&'static str {
    match update.message.text.as_str() {
        "/sensors" => handlers::handle_sensors(&token.token, &update, last_sd.inner(), storage.inner()),
        "/chat_id" => handlers::handle_chat_id(&token.token, &update),
        _ => handlers::handle_unknown_command(&token.token, &update),
    };
    return "Ok";
}

#[get("/chart")] // TODO: for debug, remove later
fn chart() -> &'static str {
    return "Not supported";
}

fn main() {
    println!("http://0.0.0.0:443/chart");
    let token = env::var("BOT_TOKEN").expect("Bot token should be defined");

    rocket::ignite()
        .mount("/", routes![index, sensors, updates, chart])
        .manage(storage::make_async_storage(String::from("sensors.db")))
        .manage(make_async_sensors_data())
        .manage(BotToken{ token })
        .launch();
}
