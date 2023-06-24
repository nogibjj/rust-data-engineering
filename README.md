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