use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    fruits
}
