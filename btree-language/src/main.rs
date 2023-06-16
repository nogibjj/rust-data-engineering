use std::collections::BTreeMap;

fn init_languages() -> BTreeMap<String, i32> {
    let mut languages = BTreeMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);

    languages
}

fn calculate_weights(languages: BTreeMap<String, i32>) -> BTreeMap<String, i32> {
    let mut years_active: BTreeMap<String, i32> = BTreeMap::new();
    for (language, year) in languages {
        years_active.insert(language, 2024 - year);
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = BTreeMap::new();
    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1; // weight between 1 and 100
        weights.insert(language.to_string(), weight);
    }

    weights
}

fn main() {
    let languages = init_languages();
    let weights = calculate_weights(languages);

    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
}
