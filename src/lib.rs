use csv::ReaderBuilder;
use rusqlite::{Connection, Result};
use std::error::Error;

pub fn create_db() -> Result<()> {
    let conn = Connection::open("SPX_Data.db")?;

    conn.execute(
        "create table if not exists SPX_info (
             Date text not null,
             Open numeric not null,
             High numeric not null,
             Low numeric not null,
             Close numeric not null,
             Adj_close numeric not null,
             Volume numeric not null
         )",
        (),
    )?;

    Ok(())
}

pub fn fill_data() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("SPX_Data.db")?;

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("../SPX.csv")?;

    while let Some(result) = reader.records().next() {
        let record = result?;

        let Date = &record[0];
        let Open = &record[1];
        let High = &record[2];
        let Low = &record[3];
        let Close = &record[4];
        let Adj_close = &record[5];
        let Volume = &record[6];
        

        if let Err(err) = conn.execute(
            "INSERT INTO SPX_info (Date, Open, High, Low, Close, Adj_close, Volume) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            &[&Date, &Open, &High, &Low, &Close, &Adj_close, &Volume],
        ) {
            eprintln!("Error inserting row: {}", err);
        }
    }

    Ok(())
}

pub fn use_query(statement: String) -> Result<()> {
    let conn = Connection::open("SPX_Data.db")?;

    let mut stmt = conn.prepare(&statement.to_string())?;

    /*let _rows = stmt.query_map([], |row|{
        let date: String = row.get(0)?;
        let open: u32 = row.get(1)?;
        println!("Date: {}, Open: {}", date, open);
        Ok(())
    })?;*/

    let mut rows = stmt.query([]).unwrap();

    while let Some(row) = rows.next()? {
        println!("{:?}", row);
    }

    Ok(())
}

#[test]
fn test_database_exists() {
    use std::path::Path;

    let path = Path::new("SPX_Data.db");
    assert!(path.exists());
}

#[test]
fn test_query_works() {
    let query = "SELECT * FROM SPX_info WHERE Volume > 6000000000";
    let _ = use_query(query.to_string());
}
