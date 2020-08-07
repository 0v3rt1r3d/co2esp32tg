use super::tgapi;
use super::storage;
use super::chart;
use super::utils;


pub fn handle_index(storage: &storage::StoragePtr) -> String {
    let locked = storage.lock();
    let storage = locked.unwrap();
    return format!(
        "Db size: {:.2} Mb; Sensors:\n{}",
        storage.db_size_mb(),
        storage
            .read().unwrap()
            .iter()
            .map(|data| {
                format!(
                    "{}, {}, {}, {}, {}",
                    data.timestamp,
                    storage::to_str(data.co2),
                    storage::to_str(data.humidity),
                    storage::to_str(data.pressure),
                    storage::to_str(data.temperature),
                )
            })
            .collect::<std::vec::Vec<String>>()
            .join("\n")
    );
}

pub fn handle_sensors(
    token: &String,
    update: &tgapi::Update,
    last_sd: &std::sync::Arc<std::sync::Mutex<Option::<storage::SensorsData>>>,
    storage: &storage::StoragePtr
) -> &'static str {
    let locked_value = match last_sd.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner()
    };
    let opt = locked_value.as_ref();
    let cloned = opt.cloned();
    let last_sd = match cloned {
        Some(s) => s,
        None => return "no"
    };
    let formatted_date = utils::parse_time(last_sd.timestamp.into()).to_string();

    tgapi::send_message(
        &token,
        &update.message.chat.id.to_string(),
        &format!("
*timestamp*: {} UTC
*temperature*: {:.1} C
*humidity*: {:.2}%
*co2*: {:.2} ppm
*pressure*: {:.2} ???
*database size*: {:.2} Mb
            ",
                 formatted_date,
                 last_sd.temperature.unwrap(),
                 last_sd.humidity.unwrap(),
                 last_sd.co2.unwrap(),
                 last_sd.pressure.unwrap(),
                 (*storage.lock().unwrap()).db_size_mb()
        )
            .replace("-", "\\\\-")
            .replace(".", "\\\\.")
    );

    return "Ok";
}

fn send_chart(
    token: &String,
    chat_id: &String,
    title: &'static str,
    filename: &'static str,
    x: &std::vec::Vec<i64>,
    y: &std::vec::Vec<f64>
) {
    chart::make_chart_encoded_base64(title, filename, x, y);
    tgapi::send_image(token, chat_id, filename);
}

// TODO: make it worked
pub fn handle_sensors_hist(
    token: &String,
    update: &tgapi::Update,
    storage: &storage::StoragePtr
) -> &'static str {
    let values = (*storage.lock().unwrap()).read().unwrap();
    let times = values.iter().map(|it| {it.timestamp}).collect();
    let chat_id = update.message.chat.id.to_string().clone();

    send_chart(
        &token,
        &chat_id,
        "Pressure, ???",
        "pressure.png",
        &times,
        &values.iter().map(|it| {
            match it.pressure {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );

    send_chart(
        token,
        &chat_id,
        "Humidity, %",
        "humidity.png",
        &times,
        &values.iter().map(|it| {
            match it.humidity {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );

    send_chart(
        token,
        &chat_id,
        "co2, ppm",
        "co2.png",
        &times,
        &values.iter().map(|it| {
            match it.co2 {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );

    send_chart(
        &token,
        &chat_id,
        "temperature, C",
        "temperature.png",
        &times,
        &values.iter().map(|it| {
            match it.temperature {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );

    return "Ok";
}

pub fn handle_unknown_command(token: &String, update: &tgapi::Update) -> &'static str {
    tgapi::send_message(
        &token,
        &update.message.chat.id.to_string(),
        "Unknown command"
    );
    return "Ok";
}

pub fn handle_chat_id(token: &String, update: &tgapi::Update) -> &'static str {
    tgapi::send_message(
        token,
        &update.message.chat.id.to_string(),
        &format!("Your `chat_id` is: `{}`", update.message.chat.id)
    );
    return "Ok";
}
