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
mod utils;


struct BotToken {
    token: String
}

#[get("/")]
fn index(storage: State<storage::StoragePtr>) -> String {
    return handlers::handle_index(&*storage);
}

#[post("/sensors", data = "<data>", format="json")]
fn sensors(
    data: Json<storage::SensorsData>,
    storage: State<storage::StoragePtr>
) -> &'static str {
    let locked = storage.lock().unwrap();
    locked.save_sensors(&data);
    return "Ok";
}

#[post("/updates", data = "<update>", format="json")]
fn updates(
    update: Json<tgapi::Update>,
    storage: State<storage::StoragePtr>,
    token: State<BotToken>
) -> &'static str {
    match update.message.text.as_str() {
        "/erase" => handlers::handle_erase(&token.token, &update, storage.inner()),
        "/sensors" => handlers::handle_sensors(&token.token, &update, storage.inner()),
        "/sensors_hist" => handlers::handle_sensors_hist(&token.token, &update, storage.inner()),
        "/start" => handlers::handle_start(&token.token, &update),
        "/chat_id" => handlers::handle_chat_id(&token.token, &update),
        _ => handlers::handle_unknown_command(&token.token, &update),
    };
    return "Ok";
}

fn main() {
    let token = env::var("BOT_TOKEN").expect("Bot token should be defined");

    rocket::ignite()
        .mount("/", routes![index, sensors, updates])
        .manage(storage::make_async_storage(String::from("sensors.db")))
        .manage(BotToken{ token })
        .launch();
}
