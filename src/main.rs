use polars::prelude::*;
use std::env;
use std::error::Error;

fn parse_and_prepare_df() -> Result<DataFrame, Box<dyn Error>> {
    // Specify non-city regions in the dataset to remove
    let non_cities = Series::new(
        "non_cities".into(),
        &[
            "California",
            "GreatLakes",
            "Midsouth",
            "Northeast",
            "Plains",
            "SouthCentral",
            "Southeast",
            "TotalUS",
            "West",
        ],
    );

    // Parse data
    let q = LazyCsvReader::new("data/avocado.csv")
        .with_has_header(true)
        .finish()?
        // Select relevant columns
        .select([
            col("Date")
                .str()
                .to_date(StrptimeOptions::default())
                .alias("date"),
            col("region"),
            col("type"),
            col("AveragePrice").alias("price"),
            (col("4046") + col("4225") + col("4770"))
                .cast(DataType::Int64)
                .alias("volume"),
        ])
        // Filter for conventional avocados and cities only
        .filter(col("type").eq(lit("conventional")))
        .filter(col("region").is_in(lit(non_cities)).not())
        // Select relevant columns
        .select([col("date"), col("region"), col("price")])
        // Sort by date and region
        .sort(["date", "region"], Default::default());

    let df = q.collect()?;

    Ok(df)
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("POLARS_FMT_MAX_ROWS", "20");
    env::set_var("POLARS_FMT_MAX_COLS", "20");

    let df = parse_and_prepare_df()?;
    println!("{}", df);

    Ok(())
}
