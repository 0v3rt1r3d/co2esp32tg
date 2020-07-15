#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate dotenv;
extern crate plotters;

use rocket::State;
use serde_json;

mod storage;

type StoragePtr = std::sync::Arc<std::sync::Mutex<storage::Storage>>;

fn make_async_storage() -> StoragePtr {
    std::sync::Arc::new(std::sync::Mutex::new(storage::Storage::new(String::from("sensors.db"))))
}

#[get("/")]
fn index(storage: State<StoragePtr>) -> String {
    // let storage = storage.lock();
    // let data = storage.unwrap();
    return format!(
        "Sensors:\n{}",
        (*storage.lock().unwrap())
            .read().unwrap()
            .iter()
            .map(|data| {
                format!(
                    "{}, {}, {}, {}, {}",
                    data.timestamp,
                    match data.co2 {
                        Some(value) => value.to_string(),
                        None => String::from("NULL")
                    },
                    match data.humidity {
                        Some(value) => value.to_string(),
                        None => String::from("NULL")
                    },
                    match data.pressure {
                        Some(value) => value.to_string(),
                        None => String::from("NULL")
                    },
                    match data.temperature {
                        Some(value) => value.to_string(),
                        None => String::from("NULL")
                    },
                )
            })
            .collect::<std::vec::Vec<String>>()
            .join("\n")
        );
}

#[post("/sensors", data = "<data>")]
fn sensors(data: String, storage: State<StoragePtr>) ->&'static str {
    let data: storage::SensorsData = serde_json::from_str(&data).unwrap();
    (*storage.lock().unwrap()).save_sensors(data);
    return "Ok";
}

#[post("/updates", data = "<data>")]
fn updates(data: String, storage: State<StoragePtr>) ->&'static str {
    // (*storage.lock().unwrap()).push(data);
    return "Did nothing";
}

#[get("/chart")]
fn chart() -> String {
    use plotters::prelude::*;
    let root = BitMapBackend::new(
        "/Users/overtired/Desktop/0.png",
        (640, 480)
    ).into_drawing_area();
    root.fill(&WHITE);
    let mut chart1 = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(-1f32..1f32, -0.1f32..1f32)
        .expect("NO");

    chart1.configure_mesh().draw();

    chart1
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        )).expect("No2")
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart1
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().expect("No3");

    return String::from("Ok");
}

fn main() {
    println!("http://0.0.0.0:443/chart");
    rocket::ignite()
        .mount("/", routes![index, sensors, updates, chart])
        .manage(make_async_storage())
        .launch();
}
