#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate dotenv;
extern crate plotters;

use rocket::State;

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

// fn establish_connection() -> SqliteConnection {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     SqliteConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }


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
    println!("http://0.0.0.0:443/chart");
    rocket::ignite()
        .mount("/", routes![index, sensors, updates, chart])
        .manage(make_async_storage())
        .launch();
}
