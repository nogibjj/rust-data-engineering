extern crate rasciigraph;

use rasciigraph::{plot, Config};

fn main() {
    let cities = vec!["Lisbon", "Madrid", "Paris", "Berlin", "Copenhagen", "Stockholm", "Moscow"];
    let distances_travelled = vec![0.0, 502.56, 1053.36, 2187.27, 2636.42, 3117.23, 4606.35];

    println!("{}", cities.join(" > "));

    println!(
        "{}",
        plot(
            distances_travelled.into_iter().map(|d| d as f64).collect(),
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("Travelled distances (km)".to_string())
        )
    );
}
