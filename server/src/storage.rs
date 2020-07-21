use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct SensorsData {
    pub timestamp: u32,
    pub co2: Option<f64>,
    pub humidity: Option<f64>,
    pub pressure: Option<f64>,
    pub temperature: Option<f64>
}

pub struct Storage {
    connection: Connection
}

// TODO: more correct?
fn to_opt<T>(result: Result<T>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(_) => None
    }
}

fn to_str<T: std::string::ToString>(option: Option<T>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => String::from("NULL")
    }
}

impl Storage {
    pub fn new(database_url: String) -> Storage {
        let connection = Connection::open(database_url).expect("failed to connect");
        connection.execute(
            "CREATE TABLE IF NOT EXISTS sensors (
                timestamp INTEGER PRIMARY KEY,
                co2 REAL,
                humidity REAL,
                pressure REAL,
                temperature REAL
            )",
            params![],
        ).expect("WTF!?");
        Storage { connection }
    }

    pub fn save_sensors(&self, data: SensorsData) {
        println!("{:?}", data);
        self.connection.execute(
            "INSERT INTO sensors (
                timestamp, 
                co2,
                humidity,
                pressure,
                temperature
            )
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                data.timestamp.to_string(),
                to_str(data.co2),
                to_str(data.humidity),
                to_str(data.pressure),
                to_str(data.temperature)
            ],
        ).expect("Sql request failed");
    }

    pub fn read(&self) -> Result<std::vec::Vec<SensorsData>> {
        let mut request = self.connection
            .prepare("
            SELECT 
                timestamp,
                co2,
                humidity,
                pressure,
                temperature
            FROM sensors")
            .expect("Failed to get request");
        let iter = request.query_map(params![], |row| {
            Ok(SensorsData {
                timestamp: row.get::<usize, u32>(0).unwrap(),
                co2: to_opt(row.get::<usize, f64>(1)),
                humidity: to_opt(row.get::<usize, f64>(2)),
                pressure: to_opt(row.get::<usize, f64>(3)),
                temperature: to_opt(row.get::<usize, f64>(4))
            })
        }).expect("No");
        Ok(iter.map(|data| {data.unwrap()}).collect::<std::vec::Vec<SensorsData>>())
    }
}
