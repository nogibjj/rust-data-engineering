use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use fruit_salad_maker::create_fruit_salad;

fn read_fruits_from_file(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut fruits = Vec::new();

    for line in reader.lines() {
        let line = line?;
        fruits.extend(line.split(',').map(|s| s.trim().to_string()));
    }
    Ok(fruits)
}

fn main() -> io::Result<()> {
    loop {
        // This reads the fruits from the file in each iteration of the loop.
        // In other languages, if not properly managed, this could cause a memory leak.
        // But in Rust, the memory used for the list of fruits is automatically deallocated at the end of each loop iteration.
        let fruits = read_fruits_from_file(Path::new("fruits.csv"))?;

        let fruit_salad = create_fruit_salad(fruits);

        println!(
            "Created Fruit salad with {} fruits: {:?}",
            fruit_salad.len(),
            fruit_salad
        );
    }
}
