use rusqlite::{Connection, Result,params};
use csv::Reader;

pub fn convert_csv_to_sql(dataset: &str) -> Result<String> {
    let conn = Connection::open("zg105.db")?;

    conn.execute("DROP TABLE IF EXISTS sample", [])?;
    conn.execute(
        "CREATE TABLE sample (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            grade REAL
        )",
        [],
    )?;

    let mut rdr = Reader::from_path(dataset).expect("Failed to read dataset");
    let mut st = conn.prepare(
        "INSERT INTO sample (
            name,
            grade
        ) 
        VALUES (?, ?)",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                st.execute(&[&record[1], &record[2]])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV: {:?}", err);
            }
        }
    }

    Ok("zg105".to_string())
}

pub fn query_crud(query: &str) -> Result<()> {
    let conn = Connection::open("zg105.db")?;
    // Read operation
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;

        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i64>(0)?, // Assuming an "id" column of type INTEGER
                row.get::<usize, String>(1)?, 
                row.get::<usize, f64>(2)?, 
            ))
        })?;

        for result in results {
            match result {
                Ok((
                    id,
                    name,
                    grade,
                )) => {
                    println!(
                        "Result: id={}, name={}, grade={}",
                        id,
                        name,
                        grade,
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // other CUD operations
        let _num_affected_rows = conn.execute_batch(query)?;
    }
    Ok(())
}