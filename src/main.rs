use env_logger::Env;
use log::info;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::UnGraph;
use petgraph::visit::EdgeRef;
use polars::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn parse_and_prepare_df() -> Result<DataFrame, Box<dyn Error>> {
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

fn convert_and_diff_df(long_df: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
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

fn calc_corr_df(wide_df: DataFrame) -> Result<(Vec<PlSmallStr>, DataFrame), Box<dyn Error>> {
    info!("Calculating correlation matrix");

    // Get region names
    let regions = wide_df.get_column_names_owned();

    // Find pairwise Pearson correlation
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

    info!("corr has {}", corr);

    Ok((regions, corr))
}

fn make_graph(regions: &[PlSmallStr], corr: &DataFrame) -> UnGraph<(), f64> {
    info!("Making graph");

    let mut graph = UnGraph::<(), f64>::new_undirected();

    // Create nodes
    let nodes = (0..regions.len())
        .map(|_| graph.add_node(()))
        .collect::<Vec<_>>();

    // Add edges
    nodes.iter().enumerate().for_each(|(i, lhs)| {
        nodes.iter().enumerate().for_each(|(j, rhs)| {
            // Query corr matrix for corr
            let corr = corr
                .column(format!("{}, {}", regions[i], regions[j]).as_str())
                .unwrap()
                .get(0)
                .unwrap()
                .try_extract::<f64>()
                .unwrap();
            // Set weight to 1 / corr**2
            let weight = f64::powf(corr, -2.0);

            graph.add_edge(*lhs, *rhs, weight);
        })
    });

    info!(
        "graph has {} nodes and {} edges",
        graph.node_count(),
        graph.edge_count()
    );

    graph
}

fn find_mst(graph: &UnGraph<(), f64>) -> UnGraph<(), f64> {
    info!("Finding mst");

    let mst = UnGraph::<(), f64>::from_elements(min_spanning_tree(&graph));

    info!(
        "mst has {} nodes and {} edges",
        mst.node_count(),
        mst.edge_count()
    );

    mst
}

fn describe_and_export_mst(
    regions: &[PlSmallStr],
    mst: &UnGraph<(), f64>,
) -> Result<(), Box<dyn Error>> {
    info!("Describing and exporting mst");

    let mut file = File::create("data/mst.txt")?;

    writeln!(file, "{} {}", mst.node_count(), mst.edge_count())?;

    // Describe and export nodes
    regions.iter().for_each(|region| {
        println!("{}", region);
        writeln!(file, "{}", region).unwrap();
    });

    // Describe and export edges
    mst.edge_references().for_each(|edge| {
        let u = edge.source().index();
        let v = edge.target().index();
        let w = edge.weight();

        println!("{}, {}, {}", regions[u], regions[v], w);
        writeln!(file, "{} {} {}", u, v, w).unwrap();
    });

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let long_df = parse_and_prepare_df()?;

    let wide_df = convert_and_diff_df(&long_df)?;

    let (regions, corr) = calc_corr_df(wide_df)?;

    let graph = make_graph(&regions, &corr);

    let mst = find_mst(&graph);

    describe_and_export_mst(&regions, &mst)?;

    Ok(())
}
