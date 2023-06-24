[![Clippy](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml)


# Rust Data Engineering

Projects for Rust Data Engineering Coursera course.
Website for projects here: [https://nogibjj.github.io/rust-data-engineering/](https://nogibjj.github.io/rust-data-engineering/)


## Feedback

* Any suggestions or feedback?  Feel free file a ticket.

## Labs (in sequential Order)

### Week 1- Rust Collections

#### Sequences

* Print Rust data structures:  `cd print-data-structs && cargo run`
* Vector Fruit Salad:  `cd vector-fruit-salad && cargo run`
* VecDeque Fruit Salad: `cd vecdeque-fruit-salad && cargo run`
* Linkedin List Fruit Salad: `cd linked-list-fruit-salad && cargo run`
* Fruit Salad CLI: `cd cli-salad && cargo run -- --number 3`

#### Maps

* Hashmap language comparison: `cd hashmap-language && cargo run`
* BTreeMap language comparison: `cd BTreeMap-language && cargo run`

#### Sets

* HashSet fruits:  `cd hashset-fruit && cargo run`
* BTreeSet fruits: `cd btreeset-fruit && cargo run`

#### Misc

* Binary Heap Fruit Salad with Fig Priority: `cd binaryheap-fruit && cargo run`

### Week 2-Rust Safety and Security 

* mutable fruit salad:  `cd mutable-fruit-salad && cargo run`
* cli customize fruit salad: `cd cli-customize-fruit-salad && cargo run -- fruits.csv` or `cargo run -- --fruits "apple, pear"`
* data race example:  `cd data-race && cargo run` (will not compile because of data race)


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

### Week 4-Rust


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