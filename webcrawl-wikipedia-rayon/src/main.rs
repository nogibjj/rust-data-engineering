/*
* The Page struct required a type implementing HttpClient trait.
* Using just a String caused a compiler error.
*
* The fix was to use the Client struct from wikipedia crate,
* which implements HttpClient.
*
* Constructing Page using Client instead of String satisfied the
* HttpClient bound. This fixed the compilation error.
*
* The key points:
* - Page requires a type implementing HttpClient
* - Pass in Client instead of String
* - Client implements HttpClient, so compiles correctly
*
* By using the right type for Page construction, it builds
* and runs properly.
*/

use wikipedia::http::default::Client;
use wikipedia::Page;
use wikipedia::Wikipedia;

struct ProcessedPage {
    title: String,
    data: String,
}

const PAGES: [&str; 9] = [
    "Giannis Antetokounmpo",
    "James Harden",
    "Russell Westbrook",
    "Stephen Curry",
    "Kevin Durant",
    "LeBron James",
    "Kobe Bryant",
    "Michael Jordan",
    "Shaquille O'Neal",
];

fn process_page(page: &Page<Client>) -> ProcessedPage {
    let title = page.get_title().unwrap();
    let content = page.get_content().unwrap();
    ProcessedPage {
        title,
        data: content,
    }
}

//times how long it takes to process the pages and total time
fn main() {
    //start timer
    let start = std::time::Instant::now();
    let wikipedia = Wikipedia::<Client>::default();
    let pages: Vec<_> = PAGES
        .iter()
        .map(|&p| wikipedia.page_from_title(p.to_string()))
        .collect();

    let processed_pages: Vec<_> = pages.iter().map(|p| process_page(p)).collect();
    for page in processed_pages {
        //time how long it takes to process each page
        let start_page = std::time::Instant::now();

        println!("Title: {}", page.title);
        //grab first sentence of the page
        let first_sentence = page.data.split('.').next().unwrap();
        println!("First sentence: {}", first_sentence);
        //count the number of words in the page
        let word_count = page.data.split_whitespace().count();
        println!("Word count: {}", word_count);
        //prints time it took to process each page
        println!("Page time: {:?}", start_page.elapsed());
    }
    //descriptive statistics of: total time, average time per page, and total number of pages, as well as the number of threads used
    println!("Total time: {:?}", start.elapsed());
    println!(
        "Average time per page: {:?}",
        start.elapsed() / PAGES.len() as u32
    );
    println!("Total number of pages: {}", PAGES.len());
    println!("Number of threads: {}", rayon::current_num_threads());
}
