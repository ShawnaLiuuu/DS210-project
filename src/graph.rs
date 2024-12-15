use log::info;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::UnGraph;
use polars::prelude::*;

pub fn make_graph(regions: &[PlSmallStr], corr: &DataFrame) -> UnGraph<(), f64> {
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

pub fn find_mst(graph: &UnGraph<(), f64>) -> UnGraph<(), f64> {
    info!("Finding mst");

    let mst = UnGraph::<(), f64>::from_elements(min_spanning_tree(&graph));

    info!(
        "mst has {} nodes and {} edges",
        mst.node_count(),
        mst.edge_count()
    );

    mst
}
