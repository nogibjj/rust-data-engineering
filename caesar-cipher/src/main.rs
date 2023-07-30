/*
This code defines two functions: encrypt and decrypt.
The encrypt function takes a plaintext string and a shift value,
and returns the ciphertext string.
The decrypt function takes a ciphertext string and a shift value,
and returns the plaintext string.

*/

use caesar_cipher::decrypt;
use caesar_cipher::encrypt;

fn main() {
    let plaintext = "the quick brown fox jumps over the lazy dog";
    let shift = 3;
    let ciphertext = encrypt(plaintext, shift);
    let decrypted_text = decrypt(&ciphertext, shift);
    println!("Plaintext: {}", plaintext);
    println!("Ciphertext: {}", ciphertext);
    println!("Decrypted text: {}", decrypted_text);
}
