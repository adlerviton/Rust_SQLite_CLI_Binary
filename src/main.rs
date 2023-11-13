use clap::Parser;
use sql_runner::{create_db, fill_data, use_query};
use std::time::Instant;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Query to Execute"
)]

struct Opts {
    #[clap(long)]
    query: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let query = opts.query;

    let now = Instant::now();
    {
        let _ = create_db().map_err(|e| format!("Error creating database: {}", e));
        println!("Database created successfully");
        let _ = fill_data().map_err(|e| format!("Error filling data: {}", e));
        println!("Data filled successfully");
        let _ = use_query(query).map_err(|e| format!("Error executing query: {}", e));
        println!("Query executed successfully");
    }

    let elapsed = now.elapsed();
    println!("Time took: {:.2?}", elapsed);
}
