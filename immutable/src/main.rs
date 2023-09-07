/* An example of an immutable list */

//This will generate a compliation error because mut is not needed
fn main(){
// a vector of immutable integers
let mut v = vec![1, 2, 3];
//print all the elements in the vector
for i in &v {
    println!("{}", i);
}
}
