/* Calculator that uses an immutable vector
Lets make our vector a constant so that we can use it in our tests
*/

// a vector of immutable integers
const V: [i32; 3] = [1, 2, 3];

// add all the elements in the vector
pub fn add() -> i32 {
    let mut sum = 0;
    for i in &V {
        sum += i;
    }
    sum
}

/*These are the tests */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(), 6);
    }
}