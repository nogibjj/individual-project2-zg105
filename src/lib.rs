use std::fs;
use reqwest::blocking::Client;
use std::fs::OpenOptions;
use std::io::Write;
use rusqlite::{Connection, Result,params};

pub fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

pub fn convert_csv_to_sql(dataset: &str) -> Result<String> {
    let conn = Connection::open("zg105.db")?;

    conn.execute("DROP TABLE IF EXISTS sample", [])?;
    conn.execute(
        "CREATE TABLE sample (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            Grade REAL
        )",
        [],
    )?;

    let mut rdr = Reader::from_path(dataset).expect("Failed to read dataset");
    let mut stmt = conn.prepare(
        "INSERT INTO iris (
            sepal_length, 
            sepal_width, 
            petal_length, 
            petal_width, 
            species
        ) 
        VALUES (?, ?, ?, ?, ?)",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute(&[&record[0], &record[1], &record[2], &record[3], &record[4]])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    Ok("IrisDataDB".to_string())
}