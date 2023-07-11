use community_detection::TWITTER_USERNAMES;
use petgraph::algo::kosaraju_scc;
use petgraph::prelude::*;
use std::collections::HashMap;

fn main() {
    // Create a new directed Graph
    let mut graph = DiGraph::<&str, &str>::new();

    // Create a HashMap to store node indices by user name
    let mut nodes = HashMap::new();

    // Iterate over the data to populate the graph
    for window in TWITTER_USERNAMES.windows(2) {
        let user = window[0];
        let mention = window[1];

        // Add the nodes to the graph and to the HashMap
        let user_node = *nodes.entry(user).or_insert_with(|| graph.add_node(user));
        let mention_node = *nodes
            .entry(mention)
            .or_insert_with(|| graph.add_node(mention));

        // Add the edge to the graph
        graph.add_edge(user_node, mention_node, "retweets");
    }

    // Use the Kosaraju's algorithm to detect strongly connected components
    let scc = kosaraju_scc(&graph);
    for component in scc {
        println!("{} nodes in community discovered", component.len());
        let usernames: Vec<&str> = component
            .iter()
            .map(|&node_index| graph[node_index])
            .collect();
        println!("{:?}", usernames);
    }
}
