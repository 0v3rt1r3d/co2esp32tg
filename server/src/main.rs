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
    return "Did nothing";
}

fn save_chart(
    filename: String,
    title: String,
    x: std::vec::Vec<u32>,
    y: std::vec::Vec<f64>,
) {
    use plotters::prelude::*;
    let first = x.first().unwrap();
    let x : std::vec::Vec<f32> = x.iter().map(|it| (it - first) as f32).collect();


    let root = BitMapBackend::new(&filename, (640, 480)).into_drawing_area();
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
    
    
    println!("x: {:?}", x.clone().into_iter().map(|a| a as f32).collect::<std::vec::Vec<f32>>());
    println!("y: {:?}", y.clone().into_iter().map(|a| a as f32).collect::<std::vec::Vec<f32>>());
    println!("x,y: {:?}", x.into_iter().map(|a| a as f32).zip(y.into_iter().map(|a| a as f32)).collect::<std::vec::Vec<(f32, f32)>>());
}

#[get("/chart")]
fn chart(storage: State<StoragePtr>) -> String {
    let values = (*storage.lock().unwrap()).read().unwrap();

    println!("Values: {:?}", values.iter().map(|it| {it.timestamp as f32}).collect::<std::vec::Vec<f32>>());
    
    save_chart(
        String::from("/Users/overtired/Desktop/pressure.png"),
        String::from("pressure"),
        values.iter().map(|it| {it.timestamp}).collect(),
        values.iter().map(|it| {
            match it.pressure {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );
    save_chart(
        String::from("/Users/overtired/Desktop/humidity.png"),
        String::from("humidity"),
        values.iter().map(|it| {it.timestamp}).collect(),
        values.iter().map(|it| {
            match it.humidity {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );
    save_chart(
        String::from("/Users/overtired/Desktop/co2.png"),
        String::from("co2"),
        values.iter().map(|it| {it.timestamp}).collect(),
        values.iter().map(|it| {
            match it.co2 {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );
    save_chart(
        String::from("/Users/overtired/Desktop/temperature.png"),
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

fn main() {
    println!("{:?}", 1594941781u32 as f64);
    println!(
        "{}\n{}\n{}\n{}",
        String::from("file:///Users/overtired/Desktop/pressure.png"),
        String::from("file:///Users/overtired/Desktop/humidity.png"),
        String::from("file:///Users/overtired/Desktop/co2.png"),
        String::from("file:///Users/overtired/Desktop/temperature.png"),
    );
    println!("http://0.0.0.0:443/chart");
    rocket::ignite()
        .mount("/", routes![index, sensors, updates, chart])
        .manage(make_async_storage())
        .launch();
}
