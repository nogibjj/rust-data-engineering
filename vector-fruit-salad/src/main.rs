use rand::prelude::SliceRandom; // Corrected import for SliceRandom trait
use std::io;

fn main() {
    let mut fruit: Vec<String> = vec![
        "Orange".to_string(),
        "Fig".to_string(),
        "Pomegranate".to_string(),
        "Cherry".to_string(),
        "Apple".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
    ];

    let predefined_fruits = vec![
        "Mango".to_string(),
        "Pineapple".to_string(),
        "Banana".to_string(),
        "Strawberry".to_string(),
        "Grapes".to_string(),
    ];

    println!("Enter a fruit to add to the salad (type 'done' to stop):");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "done" {
            break;
        }
        // Convert input to a String and push to the vector
        fruit.push(input.to_string());
    }

    // Add a specific number of random fruits from the predefined list
    let mut rng = rand::thread_rng();
    let num_random_fruits = 3; // Specify how many random fruits to add
    for _ in 0..num_random_fruits {
        if let Some(random_fruit) = predefined_fruits.choose(&mut rng) {
            fruit.push(random_fruit.to_string());
        }
    }

    // Shuffle the fruit and print the result
    fruit.shuffle(&mut rng);

    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // Use the `choose` method to select a random fruit from the salad
    if let Some(random_fruit) = fruit.choose(&mut rng) {
        println!("Randomly selected fruit from the salad: {}", random_fruit);
    } else {
        println!("The fruit salad is empty!");
    }
}
