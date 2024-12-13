[![Clippy](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml)


# Rust Data Engineering

Projects for Rust Data Engineering Coursera course.
Website for projects here: [https://nogibjj.github.io/rust-data-engineering/](https://nogibjj.github.io/rust-data-engineering/)

## Environments

* Works with both AWS CodeCatalyst and GitHub Codespaces

## Feedback

* Any suggestions or feedback?  Feel free file a ticket.

## Labs (in sequential Order)

### Week 1- Rust Data Structures: Collections

#### Sequences

* Print Rust data structures:  `cd print-data-structs && cargo run`
* Vector Fruit Salad:  `cd vector-fruit-salad && cargo run`
* VecDeque Fruit Salad: `cd vecdeque-fruit-salad && cargo run`
* Linkedin List Fruit Salad: `cd linked-list-fruit-salad && cargo run`
* Fruit Salad CLI: `cd cli-salad && cargo run -- --number 3`

#### Maps

* HashMap frequency counter: `cd hashmap-count && cargo run`
* HashMap language comparison: `cd hashmap-language && cargo run`
* BTreeMap language comparison: `cd btree-language && cargo run`

#### Sets

* HashSet fruits:  `cd hashset-fruit && cargo run`
* BTreeSet fruits: `cd btreeset-fruit && cargo run`

#### Misc

* Binary Heap Fruit Salad with Fig Priority: `cd binaryheap-fruit && cargo run`

### Week 2-Safety, Security, and Concurrency with Rust

* mutable fruit salad:  `cd mutable-fruit-salad && cargo run`
* cli customize fruit salad: `cd cli-customize-fruit-salad && cargo run -- fruits.csv` or `cargo run -- --fruits "apple, pear"`
* data race example:  `cd data-race && cargo run` (will not compile because of data race)

#### Ciphers vs Encryption

The main differences between ciphers and encryption algorithms:

* Ciphers operate directly on the plaintext, substituting or transposing the letters mathematically. Encryption algorithms operate on the binary data representations of the plaintext.

* Ciphers typically have a small key space based on simple operations like letter mappings or transposition rules. Encryption algorithms use complex math and very large key sizes.

* Ciphers provide security through obscuring letter frequencies but are still vulnerable to cryptanalysis. Encryption algorithms rely on computational hardness assumptions.

* Ciphers only handle textual data well. Encryption algorithms can handle all binary data like images, video, etc.

In summary:

* Ciphers like homophonic substitution operate directly on textual plaintext with simple math operations and fixed small key spaces.

* Encryption algorithms like AES operate on any binary data with complex math and very large key sizes.

* Ciphers are considered obsolete for serious encryption use today due to vulnerabilities.

* Modern encryption provides provable security based on mathematical problems assumed to be computationally infeasible to solve.


#### Suggested Exercises

* _Data Race Detector_: Create a multi-threaded application that attempts to produce a data race, then show how Rust's ownership rules prevent this from occurring.

* _Memory Leak Preventer_: Build an application that would typically suffer from memory leaks in other languages, such as a complex tree structure. Rust's automatic memory management through RAII (Resource Acquisition Is Initialization) should ensure no memory leaks occur.

* _Null Pointer Safety_: Create a project demonstrating how Rust's Option<T> and Result<T, E> types are used to handle potentially null or error-producing cases safely.

* _Immutable by Default_: Design a system where mutability would cause bugs (for instance, a simulation with entities that should not be able to change once created) and show how Rust's immutability by default prevents these issues.

* _System with Lifetimes_: Show how lifetimes can prevent use-after-free bugs by building an application where objects have distinct lifetimes that must be enforced.

* _No Segfault System_: Create a project that would usually segfault in other languages, and demonstrate how Rust prevents this.

* _Web Server_: Build a small multi-threaded web server. Show how Rust's safety features prevent common bugs in concurrent programming.

* _Safe FFI_: Create a project that uses Rust's Foreign Function Interface (FFI) to interoperate with C libraries safely.

* _Safe Transmute_: Write a program that demonstrates the use of safe transmutes in Rust. This could be a good way to show how Rust can avoid undefined behavior that's common in languages like C or C++.

* _Bounds Checking_: Design a system that would typically have a lot of array bounds errors, then show how Rust's automatic bounds checking prevents these kinds of errors.

* _Immutable Concurrency_: Create a project that takes advantage of Rust's ability to share immutable data among threads without data races.

* _Command Line Application_: Build a command-line application that processes user input. Use Rust's strong type system and pattern matching to handle different types of input safely and cleanly

### Week 3-Rust Data Engineering Libraries and Tools

#### Suggested Exercises

- **CSV Data Processing**: A tool for processing large CSV files, showcasing efficient data reading, filtering, and aggregation capabilities of Rust.
- **Database Interaction**: An application that interacts with a SQL database (like PostgreSQL) using Diesel, demonstrating CRUD operations, migrations, and complex queries.
- **Data Visualization**: A CLI tool that generates graphs and charts from input data using plotters.
- **Web Scraper**: A multi-threaded web scraper that fetches and parses data from several web pages concurrently.
- **REST API Consumer**: An application that interacts with a REST API to fetch, process, and visualize data.
- **Log Parser**: A tool to parse and analyze server log files. It can extract meaningful information and statistics and provide insights about the server performance.
- **File System Analyzer**: An application that provides insights about disk usage, like the `du` command in Unix.
- **Real-Time Twitter Analysis**: A real-time tweet analysis tool that uses Twitter Stream API to fetch tweets and analyze them (for example, performing sentiment analysis).
- **Stock Market Analyzer**: An application that fetches stock market data from a free API and performs various analyses.
- **Text Analytics**: A text analytics library that provides functionalities like sentiment analysis, named entity recognition, etc.
- **Delta Lake Interaction**: A project demonstrating interaction with Delta Lake for processing large amounts of data.
- **AWS SDK usage**: A project demonstrating the use of AWS SDK in Rust for tasks such as accessing S3 buckets, performing operations on DynamoDB, etc.
- **Data Processing with Polars**: A project demonstrating how to perform large-scale data processing with the Polars library in Rust.
- **Kafka Producer/Consumer**: An application that produces and consumes messages from Kafka.
- **gRPC Microservices**: A basic microservices setup using gRPC, demonstrating how Rust can be used for backend development.
- **Apache Arrow usage**: A project demonstrating how to use Apache Arrow for columnar data processing in Rust.
- **Parquet File Processing**: An application that reads and writes Parquet files, demonstrating how Rust can be used for efficient data engineering tasks.
- **Data Engineering with TiKV**: A project demonstrating how to use TiKV, a distributed transactional key-value database built in Rust.

### Week 4-Rust


#### Suggested Exercises

- **Rust-based ETL Pipeline**: Develop an ETL (Extract, Transform, Load) pipeline using various Rust libraries to process and transfer data between different storage systems.

- **Web Scraper with Rust**: Build a concurrent web scraper that can efficiently scrape large amounts of data from web pages.

- **Rust REST API Server**: Design a REST API server in Rust that serves data from a database. Use the Diesel ORM for database interactions.

- **Real-time Data Streaming with Rust**: Implement a real-time data streaming application, processing streams of data in a concurrent manner.

- **Rust-based Data Lake**: Use the Delta Lake Rust API to create a data lake solution. Implement CRUD operations on the data lake.

- **Big Data Processing with Rust and Apache Arrow**: Use Apache Arrow to perform efficient in-memory big data processing.

- **Rust and AWS SDK**: Use the AWS SDK for Rust to interact with AWS services such as S3 and DynamoDB.

- **gRPC Service in Rust**: Implement a gRPC service in Rust that performs CRUD operations on a database.

- **Log Analyzer**: Create a log analyzer that can process large log files concurrently and provide useful insights from logs.

- **Distributed Systems with Rust**: Create a simple distributed system using Rust's concurrency features. This could be a simple key-value store or a message-passing system.

- **Rust and GraphQL**: Implement a GraphQL API in Rust using libraries like Juniper.

- **Data Serialization with Rust**: Use libraries like serde to perform data serialization and deserialization in various formats (JSON, XML, etc.)

- **Rust and Kafka**: Use Rust to interact with Kafka, implementing a producer and consumer system.

- **Data Validation Service**: Create a service that validates input data based on predefined rules. This could be a web service or a library that other services can use.

- **Rust and Machine Learning**: Use Rust machine learning libraries to implement a simple prediction model. You could use the data processed in the ETL pipeline or the data lake for this.

### Lab:  Modifying a Rust Command-Line Tool

In this lab you will gain experience extending an existing Rust project by forking and modifying a simple command-line tool.

**Steps**

1. Fork the repository at **https://github.com/nogibjj/rust-data-engineering**

2. Clone your forked repository 

3. Navigate to one of the command-line tool projects

4. Make a small modification to the tool such as:

   - Adding a new command line argument
    
   - Supporting additional input file formats
    
   - Adding more processing logic
    
   - Changing output formatting

5. Run `cargo build` to compile your changes  

6. Run `cargo run` to test your modified tool

7. Commit your changes and push to your forked repository


**Deliverable**

Submit a link to your forked repository showing the code changes.


**Goals**

This hands-on lab provides experience with:

- Forking and cloning a Rust project

- Modifying existing Rust code 

- Running `cargo build` and `cargo run`

- Version control with git

- Making a pull request (optional)


### Technical Notes

## Makefile

Each subdirectory project uses this style to make it easy to test and run

```
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run
```


## References

* [Rust Collections](https://doc.rust-lang.org/std/collections/index.html)
* [GitHub Copilot CLI](https://www.npmjs.com/package/@githubnext/github-copilot-cli)
* [Rust Fundamentals](https://github.com/alfredodeza/rust-fundamentals)
* [Rust Tutorial](https://nogibjj.github.io/rust-tutorial/)
* [Rust MLOps Template](https://github.com/nogibjj/mlops-template)
