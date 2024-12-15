use crate::corr_matrix;
use log::info;
use polars::prelude::*;
use std::error::Error;

pub fn parse_and_prepare_df() -> Result<DataFrame, Box<dyn Error>> {
    info!("Parsing and preparing df");

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

    info!("long_df has {}", long_df);

    Ok(long_df)
}

pub fn convert_and_diff_df(long_df: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    info!("Converting and diffing df");

    // Pivot to wide-form data
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
    // Calculate deltas
    .select([all().map(
        |price| {
            let price_shifted = price.shift(1);
            Ok(Some((price - price_shifted)?))
        },
        Default::default(),
    )])
    .fill_null(0)
    .collect()?;

    info!("wide_df has {}", wide_df);

    Ok(wide_df)
}

pub fn calc_corr_df(wide_df: DataFrame) -> Result<corr_matrix::CorrMatrix, Box<dyn Error>> {
    info!("Calculating correlation matrix");

    // Get region names
    let regions = wide_df.get_column_names_owned();

    // Find pairwise Pearson correlation
    let corr_df = wide_df
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

    info!("corr_df has {}", corr_df);

    Ok(corr_matrix::CorrMatrix { regions, corr_df })
}
