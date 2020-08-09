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
    storage: &storage::StoragePtr
) -> &'static str {
    let last_sd = (*storage.lock().unwrap()).read_last().unwrap();
    let formatted_date = utils::parse_time(last_sd.timestamp.into()).to_string();

    tgapi::send_message(
        &token,
        &update.message.chat.id.to_string(),
        &format!("
*timestamp*: {}
*temperature*: {:.1} C
*humidity*: {:.2}%
*co2*: {:.2} ppm
*pressure*: {:.2} hPa
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
            .replace("+", "\\\\+")
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
    chart::save_chart(title, filename, x, y);
    tgapi::send_image(token, chat_id, filename);
}

pub fn handle_sensors_hist(
    token: &String,
    update: &tgapi::Update,
    storage: &storage::StoragePtr
) -> &'static str {
    let values = (*storage.lock().unwrap()).read().unwrap();
    let times = values.iter().map(|it| {it.timestamp}).collect();
    let chat_id = update.message.chat.id.to_string().clone();

    send_chart(
        token,
        &chat_id,
        "Pressure, hPa",
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
        token,
        &chat_id,
        "Temperature, C",
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
        token,
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

pub fn handle_erase(
    token: &String,
    update: &tgapi::Update,
    storage: &storage::StoragePtr
) -> &'static str {
    (*storage.lock().unwrap()).erase_previous_weeks();
    tgapi::send_message(
        token,
        &update.message.chat.id.to_string(),
        "Old database entities were erased"
    );
    return "Ok";
}

pub fn handle_start(
    token: &String,
    update: &tgapi::Update
) -> &'static str {
    tgapi::send_message(
        token,
        &update.message.chat.id.to_string(),
        "
I am overtired's bot. I can send you:
- Send current reading from air sensors
- Send charts with air sensors values changes
- Send your current chat id (used to send notifications)
"
    );
    return "Ok";
}
