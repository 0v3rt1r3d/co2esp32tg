use super::tgapi;
use super::storage;
use super::chart;

use chrono::{NaiveDateTime, FixedOffset};

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
    let naive_dt = NaiveDateTime::from_timestamp(last_sd.timestamp.into(), 0);
    let msk_dt : chrono::DateTime<FixedOffset> = chrono::DateTime::from_utc(naive_dt, chrono::FixedOffset::east(60 * 60 * 3));
    let formatted_date = msk_dt.to_string();

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

// TODO: make it worked
pub fn handle_sensors_hist(
    token: &String,
    update: &tgapi::Update,
    storage: &storage::StoragePtr
) -> &'static str {
    let values = (*storage.lock().unwrap()).read().unwrap();

    chart::make_chart_encoded_base64(
        String::from("pressure"),
        values.iter().map(|it| {it.timestamp}).collect(),
        values.iter().map(|it| {
            match it.pressure {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );
    tgapi::send_image(token, &update.message.chat.id.to_string(), "pressure.png");

    chart::make_chart_encoded_base64(
        String::from("humidity"),
        values.iter().map(|it| {it.timestamp}).collect(),
        values.iter().map(|it| {
            match it.humidity {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );
    tgapi::send_image(token, &update.message.chat.id.to_string(), "humidity.png");

    &chart::make_chart_encoded_base64(
        String::from("co2"),
        values.iter().map(|it| {it.timestamp}).collect(),
        values.iter().map(|it| {
            match it.co2 {
                Some(v) => v,
                None => 0f64
            }
        }).collect());
    tgapi::send_image(token, &update.message.chat.id.to_string(), "co2.png");

    chart::make_chart_encoded_base64(
        String::from("temperature"),
        values.iter().map(|it| {it.timestamp}).collect::<std::vec::Vec<i64>>(),
        values.iter().map(|it| {
            match it.temperature {
                Some(v) => v,
                None => 0f64
            }
        }).collect()
    );
    tgapi::send_image(token, &update.message.chat.id.to_string(), "temperature.png");
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
