[![Clippy](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml)


# Rust Data Engineering

Projects for Rust Data Engineering Coursera course.
Website for projects here: [https://nogibjj.github.io/rust-data-engineering/](https://nogibjj.github.io/rust-data-engineering/)


## Feedback

* Any suggestions or feedback?  Feel free file a ticket.

## Labs (in sequential Order)

### Sequences

1. Print Rust data structures:  `cd print-data-structs && cargo run`
2. Vector Fruit Salad:  `cd vector-fruit-salad && cargo run`
3. VecDeque Fruit Salad: `cd vecdeque-fruit-salad && cargo run`
4. Linkedin List Fruit Salad: `cd linked-list-fruit-salad && cargo run`
5. Fruit Salad CLI: `cd cli-salad && cargo run -- --number 3`

### Maps

6. Hashmap language comparison: `cd hashmap-language && cargo run`
7. BTreeMap language comparison: `cd BTreeMap-language && cargo run`

### Sets

8. HashSet:  `cd hashset-fruit && cargo run`
9. BTreeSet: `cd btreeset-fruit && cargo run`

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