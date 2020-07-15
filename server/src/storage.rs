use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
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
                match data.co2 {
                    Some(value) => value.to_string(),
                    None => String::from("NULL")
                },
                match data.humidity {
                    Some(value) =>value.to_string(),
                    None => String::from("NULL")
                },
                match data.pressure {
                    Some(value) => value.to_string(),
                    None => String::from("NULL")
                },
                match data.temperature { // TODO: wrap a function
                    Some(value) => value.to_string(),
                    None => String::from("NULL") // TODO: use named constant
                },
            ],
        );
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
                co2: match row.get::<usize, f64>(1) {
                    Ok(value) => Some(value),
                    Err(_) => None
                },
                humidity: match row.get::<usize, f64>(2) {
                    Ok(value) => Some(value),
                    Err(_) => None
                },
                pressure: match row.get::<usize, f64>(3) {
                    Ok(value) => Some(value),
                    Err(_) => None
                },
                temperature: match row.get::<usize, f64>(4) {
                    Ok(value) => Some(value),
                    Err(_) => None
                }
            })
        }).expect("No");
        Ok(iter.map(|data| {data.unwrap()}).collect::<std::vec::Vec<SensorsData>>())
    }
}
