use crate::corr_matrix;
use log::info;
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::UnGraph;

pub fn make_graph(corr_matrix: &corr_matrix::CorrMatrix) -> UnGraph<(), f64> {
    info!("Making graph");

    let mut graph = UnGraph::<(), f64>::new_undirected();

    // Create nodes
    let nodes = (0..corr_matrix.regions.len())
        .map(|_| graph.add_node(()))
        .collect::<Vec<_>>();

    // Add edges
    nodes.iter().enumerate().for_each(|(i, lhs)| {
        nodes.iter().enumerate().for_each(|(j, rhs)| {
            // Set weight to 1 / corr**2
            let weight = f64::powf(corr_matrix.get(i, j), -2.0);

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
