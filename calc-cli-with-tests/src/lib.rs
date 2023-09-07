/*
This is a library that randomly returns fruits native to Portugual.
The style of the code is that we use a constant array of strings to store the fruits.
We then use this const in a function that later get called in the main.rs file as a CLI.

The CLI should support the following:

//the quantity of fruits to return
--count 5
*/

use rand::Rng;

// a vector of immutable strings that represent fruits native to Portugal and the Azores
const FRUITS: [&str; 10] = [
    "banana",
    "apple",
    "orange",
    "pear",
    "pineapple",
    "grape",
    "strawberry",
    "raspberry",
    "blueberry",
    "blackberry",
];

/*
return a random fruit from the FRUITS vector
Accepts a count of fruits to return
*/

pub fn get_fruits(count: u32) -> Vec<String> {
    let mut fruits = Vec::new();
    for _ in 0..count {
        let fruit = rand::thread_rng().gen_range(0..FRUITS.len());
        fruits.push(FRUITS[fruit].to_string());
    }
    fruits
}

/*Test 

A test that checks if the get_fruits function returns the correct number of fruits
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fruits() {
        let fruits = get_fruits(5);
        assert_eq!(fruits.len(), 5);
    }
}
