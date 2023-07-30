/*
This code defines a function called create_fruit_salad 
that takes a mutable vector of strings as input and returns 
a new vector of strings that contains the same elements as the input vector, 
but in a random order.
*/

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    fruits
}
