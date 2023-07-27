fn main() {
    // Create a vector of fruits.
    let fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    println!("Original fruit salad: {:?}", fruit_salad);

    // Uncommenting the following line will cause a compilation error because fruit_salad 
    // is immutable.
    // fruit_salad.push("figs");

    // To mutate the vector, we need to declare it as mutable:
    let mut fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    fruit_salad.push("figs");
    println!("Modified fruit salad: {:?}", fruit_salad);
}
