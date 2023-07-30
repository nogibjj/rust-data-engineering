use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use sha3::Digest;
use sha3::Sha3_256;
use std::collections::HashMap;

// List of phrases
static PHRASES: [&str; 10] = [
    "man can be destroyed but not defeated",
    "but man is not made for defeat",
    "a man can be destroyed but not defeated",
    "the old man was thin and gaunt",
    "everything about him was old",
    "the sail was patched with flour sacks",
    "he was an old man who fished alone",
    "the old man had taught the boy to fish",
    "the old man looked at him with his sun burned confident loving eyes",
    "his eyes were cheerful and undefeated",
];

// Generate random phrases
pub fn generate_random_phrases() -> Vec<&'static str> {
    let mut rng = thread_rng();
    let mut phrases = Vec::new();

    for &phrase in PHRASES.iter() {
        let copies = rng.gen_range(1..=3);

        for _ in 0..copies {
            phrases.push(phrase);
        }
    }

    phrases.shuffle(&mut rng);

    phrases
}

// Analyze duplicates
pub fn analyze_duplicates(phrases: &[&str]) {
    let mut hashes: HashMap<_, (usize, &str)> = HashMap::new();
    println!("Total number of phrases: {}", phrases.len());

    for phrase in phrases {
        let hash = Sha3_256::digest(phrase.as_bytes());
        let entry = hashes.entry(hash).or_insert((0, phrase));
        entry.0 += 1;
    }

    let total_unique_phrases = hashes.len();

    let mut total_unique_duplicates = 0;
    let mut total_combined_duplicates = 0;

    for (hash, (count, phrase)) in &hashes {
        if *count > 1 {
            total_unique_duplicates += 1;
            total_combined_duplicates += count - 1; // subtract one to exclude the original
            println!("{} - {} times: {}", hex::encode(hash), count, phrase);
        }
    }

    println!("Total Unique Phrases: {}", total_unique_phrases);
    println!("Total Unique Duplicates: {}", total_unique_duplicates);
    println!("Total Combined Duplicates: {}", total_combined_duplicates);
}
