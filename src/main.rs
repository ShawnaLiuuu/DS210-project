use env_logger::Env;
use log::info;
use petgraph::graph::UnGraph;
use petgraph::visit::EdgeRef;
use std::error::Error;
use std::fs::File;
use std::io::Write;

mod corr_matrix;
mod data;
mod graph;

fn describe_and_export_mst(
    corr_matrix: &corr_matrix::CorrMatrix,
    mst: &UnGraph<(), f64>,
) -> Result<(), Box<dyn Error>> {
    info!("Describing and exporting mst");

    let mut file = File::create("output/mst.txt")?;

    writeln!(file, "{} {}", mst.node_count(), mst.edge_count())?;

    // Describe and export nodes
    corr_matrix
        .regions
        .iter()
        .for_each(|region| println!("{}", region));

    // Describe and export edges
    mst.edge_references().for_each(|edge| {
        let u = edge.source().index();
        let v = edge.target().index();
        let corr = corr_matrix.get(u, v);

        println!(
            "{}, {}, {}",
            corr_matrix.regions[u], corr_matrix.regions[v], corr
        );
        writeln!(
            file,
            "{} {} {}",
            corr_matrix.regions[u], corr_matrix.regions[v], corr
        )
        .unwrap();
    });

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let long_df = data::parse_and_prepare_df()?;

    let wide_df = data::convert_and_diff_df(&long_df)?;

    let corr_matrix = data::calc_corr_df(wide_df)?;

    let graph = graph::make_graph(&corr_matrix);

    let mst = graph::find_mst(&graph);

    describe_and_export_mst(&corr_matrix, &mst)?;

    Ok(())
}
