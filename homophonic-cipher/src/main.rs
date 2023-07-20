/*
 * Generates a list of random homophones for each lowercase letter in the
 * English alphabet. Maps each character in the plaintext to one of its 
 * random homophones to create the cipher text. Prints the plaintext, 
 * cipher text, and homophonic mapping. Returns the cipher text and 
 * homophonic mapping.
 * 
 *
 * Here is an example:
 * Plaintext: the quick brown fox jumps over the lazy dog
 * Ciphertext: acrsalgzuwxsgpeqqrjrnekrwvnnwdgfuqn
 * Mapping: {
 *     't': ['q', 'a', 'v'], 'y': ['f', 's'], 'q': ['s', 'u'], 
 *     'l': ['w', 'z', 'o'], 's': ['i', 'w', 'n'], 'b': ['u', 'f'], 
 *     'h': ['n', 'n', 'c'], 'k': ['z', 'r'], 'j': ['s', 'w', 'q'], 
 *     'x': ['g', 'g', 'q'], 'i': ['l', 'k'], 'g': ['n', 'g'], 
 *     'm': ['s', 'j', 'w'], 'p': ['k', 'r'], 'a': ['d', 'm', 'w'], 
 *     'r': ['w', 'o', 'o'], 'o': ['q', 'x', 'e'], 'e': ['n', 'r'], 
 *     'f': ['i', 'p', 'e'], 'c': ['g', 'z'], 'u': ['a', 'd', 'r'], 
 *     'v': ['h', 'f', 'k'], 'd': ['s', 'r', 'u'], 'n': ['d', 'g', 'l'], 
 *     'w': ['s', 'c'], 'z': ['g', 'b']
 * } 
 * The mapping {'t': ['q', 'a', 'v'], ...} is a part of the homophonic 
 * cipher mapping from plaintext characters to their cipher characters.
 *

 * In this specific example, the plaintext character 't' can be represented
 * in the ciphertext by either 'q', 'a', or 'v'. This introduces ambiguity 
 * into the encryption, which makes the homophonic cipher harder to break 
 * compared to simple substitution ciphers.
 *
 * The homophonic cipher is more secure than a simple substitution cipher, 
 * it is still not secure for serious cryptographic uses.
 *
 * Given the ciphertext and the mapping, you can reverse-engineer the 
 * plaintext. Let's start with the first three characters of the 
 * ciphertext: 'a', 'c', and 'r'.
 *
 * 'a': Looking at the mapping, you can see that 'a' can be a cipher for 
 * 'u' or 't', as 'u' and 't' have 'a' in their list of homophones. So 
 * the possible plaintext letters for 'a' are 'u' and 't'.
 *
 * 'c': Looking at the mapping again, 'c' can be a cipher for 'h' or 'w' 
 * since 'h' and 'w' have 'c' in their list of homophones. So the 
 * possible plaintext letters for 'c' are 'h' and 'w'.
 *
 * 'r': 'r' can be a cipher for 'e', 'o', or 't' since 'e', 'o', and 't' 
 * have 'r' in their list of homophones. So the possible plaintext 
 * letters for 'r' are 'e', 'o', and 't'.
 *
 * So the first three characters of the plaintext could be any combination
 * of the possible plaintext letters for 'a', 'c', and 'r'. For example, 
 * it could be 'u', 'h', 'e', or 't', 'w', 'o', etc.
 *
 * Remember, homophonic ciphers are designed to provide many possible 
 * plaintexts for a single ciphertext, which makes it much harder to crack
 * the code without having more information. One possible approach to 
 * decode the message is using a frequency analysis or a known-plaintext 
 * attack if you have a part of the original message. Another way is to 
 * use the context of the message if it's known.
 */

use rand::Rng;
use std::collections::HashMap;

fn homophonic_cipher(plaintext: &str) -> (String, HashMap<char, Vec<char>>) {
    let mut rng = rand::thread_rng();
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut ciphertext = String::new();
    let mut mapping: HashMap<char, Vec<char>> = HashMap::new();

    for c in &alphabet {
        let homophones: Vec<char> = (0..rng.gen_range(2..4))
            .map(|_| rng.gen_range('a'..='z'))
            .collect();
        mapping.insert(*c, homophones);
    }

    for c in plaintext.chars() {
        if let Some(c) = c.to_lowercase().next() {
            if let Some(homophones) = mapping.get(&c) {
                if let Some(&homophone) = homophones.get(rng.gen_range(0..homophones.len())) {
                    ciphertext.push(homophone);
                } else {
                    eprintln!("Error: No homophones for character {}", c);
                }
            }
        } else {
            ciphertext.push(c);
        }
    }

    println!("Plaintext: {}", plaintext);
    println!("Ciphertext: {}", ciphertext);
    println!("Mapping: {:?}", mapping);

    (ciphertext, mapping)
}

fn main() {
    let plaintext = "the quick brown fox jumps over the lazy dog";
    homophonic_cipher(plaintext);
}
