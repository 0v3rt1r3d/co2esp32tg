#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate dotenv;
extern crate plotters;

use chrono::NaiveDateTime;
use rocket::State;
use serde_json;
use std::env;
use std::str;

mod storage;

type StoragePtr = std::sync::Arc<std::sync::Mutex<storage::Storage>>;
fn make_async_storage(db_name: String) -> StoragePtr {
    std::sync::Arc::new(std::sync::Mutex::new(storage::Storage::new(db_name)))
}

// Temporary replacements till charts are not drawn
type SensorsDataPtr = std::sync::Arc<std::sync::Mutex<Option::<storage::SensorsData>>>;

fn make_async_sensors_data() -> SensorsDataPtr {
    std::sync::Arc::new(std::sync::Mutex::new(None))
}

// #[get("/")]
// fn index(storage: State<StoragePtr>) -> String {
//     return format!(
//         "Sensors:\n{}",
//         (*storage.lock().unwrap())
//             .read().unwrap()
//             .iter()
//             .map(|data| {
//                 format!(
//                     "{}, {}, {}, {}, {}",
//                     data.timestamp,
//                     storage::to_str(data.co2),
//                     storage::to_str(data.humidity),
//                     storage::to_str(data.pressure),
//                     storage::to_str(data.temperature),
//                 )
//             })
//             .collect::<std::vec::Vec<String>>()
//             .join("\n")
//         );
// }

#[get("/")]
fn index(storage: State<SensorsDataPtr>) -> String {
    let locked_value = match storage.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner()
    };
    let opt = locked_value.as_ref();
    let cloned = opt.cloned();
    let data = cloned.unwrap();

    return format!(
        "Sensors: {}, {}, {}, {}, {}",
        data.timestamp,
        storage::to_str(data.co2),
        storage::to_str(data.humidity),
        storage::to_str(data.pressure),
        storage::to_str(data.temperature),
    );
}

#[post("/sensors", data = "<data>")]
fn sensors(data: String, storage: State<SensorsDataPtr>) ->&'static str {
    let data: storage::SensorsData = serde_json::from_str(&data).unwrap();
    // (*storage.lock().unwrap()).save_sensors(data);

    // Temprorary
    let mut locked = storage.lock().unwrap();
    locked.replace(data);

    return "Ok";
}

fn send_message(
    token: &str,
    chat_id: &str,
    text: &str
) {
    let client = reqwest::blocking::Client::new();
    client.post(&format!("https://api.telegram.org/bot{}/sendMessage", token))
        .body(format!(
            "{{ \
                \"chat_id\":{}, \
                \"text\":\"{}\", \
                \"parse_mode\":\"MarkdownV2\" \
            }}",
            chat_id,
            text,
        ))
        .header("Content-Type", "application/json")
        .send()
        .unwrap();
}

#[post("/updates", data = "<body>")]
fn updates(
    body: String,
    storage: State<SensorsDataPtr>,
    token: State<BotToken>
) ->&'static str { // TODO: get rid of String, build release
    println!("{}", body);
    let update: serde_json::Value = serde_json::from_str(&body).unwrap();
    // let all_data = (*storage.lock().unwrap()).read().unwrap();
    // let last_sd = all_data.last().unwrap();
    if update["message"]["text"] == "/sensors" {
        let locked_value = match storage.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };
        let opt = locked_value.as_ref();
        let cloned = opt.cloned();
        let last_sd = cloned.unwrap();
        let formatted_date = NaiveDateTime::from_timestamp(last_sd.timestamp.into(), 0);
        
        send_message(
            &token.token,
            &update["message"]["chat"]["id"].to_string(),
            &format!(
                "The last sensors data:
                    timestamp = {};
                    temperature = {} C;
                    humidity = {};
                    co2 = {};
                    pressure = {};
                ",
                formatted_date,
                last_sd.temperature.unwrap(),
                last_sd.humidity.unwrap(),
                last_sd.co2.unwrap(),
                last_sd.pressure.unwrap()
            )
        );
    } else if update["message"]["text"] == "/chat_id" {
        send_message(
            &token.token,
            &update["message"]["chat"]["id"].to_string(),
            &format!("Your `chat_id` is: `{}`", update["message"]["chat"]["id"])
        );
    } else {
        send_message(
            &token.token,
            &update["message"]["chat"]["id"].to_string(),
            "Unknown command"
        );
    }
    send_message(
        &token.token,
        &update["message"]["chat"]["id"].to_string(),
        "Processed user command"
    );

    return "Did nothing";
}

fn make_chart_encoded_base64(
    title: String,
    x: std::vec::Vec<u32>,
    y: std::vec::Vec<f64>
) -> std::vec::Vec<u8> {
    use plotters::prelude::*;
    let first = x.first().unwrap();
    let x : std::vec::Vec<f32> = x.iter().map(|it| (it - first) as f32).collect();

    let width = 1000;
    let height = 800;

    let mut buffer: std::vec::Vec<u8> = vec![0; (width * height * 3) as usize];
    {
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        root.fill(&WHITE).expect("Filled white");

        let mut sorted_y: std::vec::Vec<f32> = y.clone().into_iter().map(|x| x as f32).collect();
        sorted_y.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut chart1 = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_ranged(
                x.first().unwrap().clone() as f32 .. x.last().unwrap().clone() as f32, 
                sorted_y.first().unwrap().clone() as f32..sorted_y.last().unwrap().clone() as f32
            )
            .expect("NO");

        chart1.configure_mesh().draw().expect("Drawing mesh");

        chart1
            .draw_series(LineSeries::new(
                x.clone().into_iter().map(|a| a as f32).zip(y.clone().into_iter().map(|a| a as f32)),
                &RED,
            )).expect("No3")
            .label("real graph");
    }

    // let img_buffer = image::ImageBuffer::<image::Rgb<u8>, std::vec::Vec<u8>>::from_raw(width, height, &buffer).unwrap();
    let img = image::RgbImage::from_raw(width, height, buffer).unwrap();
    let img = image::DynamicImage::ImageRgb8(img);
    let mut output = vec![0];
    img.write_to(&mut output, image::ImageFormat::Png).unwrap();

    // return base64::encode(output);
    return output;
}

#[get("/chart")]
fn chart(storage: State<StoragePtr>) -> String {
    let values = (*storage.lock().unwrap()).read().unwrap();

    println!("Values: {:?}", values.iter().map(|it| {it.timestamp as f32}).collect::<std::vec::Vec<f32>>());
    
    make_chart_encoded_base64(
        String::from("pressure"),
        values.iter().map(|it| {it.timestamp}).collect(),
        values.iter().map(|it| {
            match it.pressure {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );
    make_chart_encoded_base64(
        String::from("humidity"),
        values.iter().map(|it| {it.timestamp}).collect(),
        values.iter().map(|it| {
            match it.humidity {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );
    make_chart_encoded_base64(
        String::from("co2"),
        values.iter().map(|it| {it.timestamp}).collect(),
        values.iter().map(|it| {
            match it.co2 {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );
    make_chart_encoded_base64(
        String::from("temperature"),
        values.iter().map(|it| {it.timestamp}).collect::<std::vec::Vec<u32>>(),
        values.iter().map(|it| {
            match it.temperature {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );

    return String::from("Ok");
}

struct BotToken {
    token: String
}

fn main() {
    println!("http://0.0.0.0:443/chart");
    let token = env::var("BOT_TOKEN").expect("Bot token should be defined");

    rocket::ignite()
        .mount("/", routes![index, sensors, updates, chart])
        // .manage(make_async_storage(String::from("sensors.db")))
        .manage(make_async_sensors_data())
        .manage(BotToken{ token : token })
        .launch();
}
