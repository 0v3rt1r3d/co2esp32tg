use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{Duration, SystemTime};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SensorsData {
    pub timestamp: i64,
    pub co2: Option<f64>,
    pub humidity: Option<f64>,
    pub pressure: Option<f64>,
    pub temperature: Option<f64>
}

pub struct Storage {
    connection: Connection,
    db_file: String
}

fn to_opt<T>(result: Result<T>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(_) => None
    }
}

pub fn to_str<T: std::string::ToString>(option: Option<T>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => String::from("NULL")
    }
}

pub type StoragePtr = std::sync::Arc<std::sync::Mutex<Storage>>;
pub fn make_async_storage(db_name: String) -> StoragePtr {
    std::sync::Arc::new(std::sync::Mutex::new(Storage::new(db_name).unwrap()))
}

impl Storage {
    pub fn new(database_url: String) -> std::result::Result<Storage, rusqlite::Error> {
        let connection = Connection::open(&database_url).expect("failed to connect");
        connection.execute(
            "CREATE TABLE IF NOT EXISTS sensors (
                timestamp INTEGER PRIMARY KEY,
                co2 REAL,
                humidity REAL,
                pressure REAL,
                temperature REAL
            )",
            params![],
        )?;
        return Ok(Storage { connection, db_file: database_url })
    }

    pub fn save_sensors(&self, data: &SensorsData) -> std::result::Result<(), rusqlite::Error>{
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
        )?;
        return Ok(());
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
                timestamp: row.get::<usize, i64>(0).unwrap(),
                co2: to_opt(row.get::<usize, f64>(1)),
                humidity: to_opt(row.get::<usize, f64>(2)),
                pressure: to_opt(row.get::<usize, f64>(3)),
                temperature: to_opt(row.get::<usize, f64>(4))
            })
        }).unwrap();
        Ok(iter.map(|data| {data.unwrap()}).collect::<std::vec::Vec<SensorsData>>())
    }

    pub fn read_last(&self) -> Result<SensorsData> {
        let mut request = self.connection
            .prepare("
            SELECT
                timestamp,
                co2,
                humidity,
                pressure,
                temperature
            FROM sensors
            WHERE timestamp = (SELECT MAX(timestamp) FROM sensors ORDER BY timestamp DESC)
            ")
            .unwrap();
        let iter = request.query_map(params![], |row| {
            Ok(SensorsData {
                timestamp: row.get::<usize, i64>(0).unwrap(),
                co2: to_opt(row.get::<usize, f64>(1)),
                humidity: to_opt(row.get::<usize, f64>(2)),
                pressure: to_opt(row.get::<usize, f64>(3)),
                temperature: to_opt(row.get::<usize, f64>(4))
            })
        })
            .unwrap();
        Ok(iter.map(|data| {data.unwrap()}).collect::<std::vec::Vec<SensorsData>>()[0].clone())
    }

    pub fn erase_previous_month(&self) -> std::result::Result<(), rusqlite::Error> {
        let time_offset = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap() - Duration::from_secs(60 * 60 * 24 * 30)).as_secs();
        let mut request = self.connection
            .prepare(&format!(" \
                DELETE FROM sensors \
                WHERE timestamp < {}",
                time_offset
            ))?;
        request.execute(params![])?;
        return Ok(());
    }

    pub fn db_size_mb(&self) -> f64 {
        return fs::metadata(&self.db_file).unwrap().len() as f64 / 1024f64 / 1024f64;
    }
}
