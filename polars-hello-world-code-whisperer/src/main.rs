/*
Polars hello world script that uses AWS Code Catalyst and Code Whisperer
*/
use polars::prelude::*;

pub fn calculate() -> Result<DataFrame, PolarsError> {
    // Read the CSV data using CsvReader
    let df = CsvReader::new("data/iris.csv")
        .has_header(true)
        .finish()?
        .lazy()
        .collect()?;
    Ok(df)
}

fn main() {
    let df = calculate().unwrap();
    println!("{}", df);
}