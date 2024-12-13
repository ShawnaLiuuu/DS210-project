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

    let long_df = q.collect()?;

    Ok(long_df)
}

fn convert_and_diff_df(long_df: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    let wide_df = pivot::pivot_stable(
        long_df,
        ["region"],
        Some(["date"]),
        Some(["price"]),
        false,
        None,
        None,
    )?
    .lazy()
    .sort(["date"], Default::default())
    .drop(["date"])
    .select([all().map(
        |price| {
            let price_shifted = price.shift(1);
            Ok(Some((price - price_shifted)?))
        },
        Default::default(),
    )])
    .fill_null(0)
    .collect()?;

    Ok(wide_df)
}

fn calc_corr_df(wide_df: DataFrame) -> Result<(Vec<PlSmallStr>, DataFrame), Box<dyn Error>> {
    let regions = wide_df.get_column_names_owned();

    let corr = wide_df
        .lazy()
        .select(
            regions
                .iter()
                .flat_map(|lhs| {
                    regions.iter().map(move |rhs| {
                        pearson_corr(col(lhs.to_owned()), col(rhs.to_owned()))
                            .alias(format!("{}, {}", lhs, rhs))
                    })
                })
                .collect::<Vec<_>>(),
        )
        .collect()?;

    Ok((regions, corr))
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("POLARS_FMT_MAX_ROWS", "20");
    env::set_var("POLARS_FMT_MAX_COLS", "20");

    let long_df = parse_and_prepare_df()?;
    println!("{}", long_df);

    let wide_df = convert_and_diff_df(&long_df)?;
    println!("{}", wide_df);

    let (regions, corr) = calc_corr_df(wide_df)?;
    println!("{:?}", regions);
    println!("{}", corr);

    Ok(())
}
