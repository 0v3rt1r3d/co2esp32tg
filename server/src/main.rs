#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

static mut received_updates: std::vec::Vec::<std::string::String> = std::vec::Vec::<std::string::String>::new();
static mut received_sensors: std::vec::Vec::<std::string::String> = std::vec::Vec::<std::string::String>::new();

#[get("/")]
fn index() -> String {
    let updates = unsafe {
        received_updates.join("\n")
    };
    let sensors = unsafe {
        received_sensors.join("\n")
    };
    return format!("Updates:\n{}\nSensors:\n{}", updates, sensors);
}

#[post("/sensors", data = "<data>")]
fn sensors(data: String) ->&'static str {
    unsafe {
        received_sensors.push(data);
    }
    return "Ok";
}

#[post("/updates", data = "<data>")]
fn updates(data: String) ->&'static str {
    unsafe {
        received_updates.push(data);
    }
    return "Ok";
}

fn main() {
    rocket::ignite().mount("/", routes![index, sensors, updates]).launch();
}
