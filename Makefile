install:
	cargo install mdbook

build:
	mdbook build data-eng-rust-tutorial

serve:
	mdbook serve -p 8000 -n 127.0.0.1 data-eng-rust-tutorial 

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

linkcheck:
	mdbook test -L data-eng-rust-tutorial

run:
	cargo run

release:
	cargo build --release

deploy:
	#if git is not configured, configure it
	if [ -z "$(git config --global user.email)" ]; then git config --global user.email "noah.gift@gmail.com" &&\
		 git config --global user.name "Noah Gift"; fi

	#install mdbook if not installed
	if [ ! -x "$(command -v mdbook)" ]; then cargo install mdbook; fi
	@echo "====> deploying to github"
	# if worktree exists, remove it: git worktree remove --force /tmp/book
	# otherwise add it: git worktree add /tmp/book gh-pages
	if [ -d /tmp/book ]; then git worktree remove --force /tmp/book; fi
	git worktree add -f /tmp/book gh-pages
	mdbook build small-rust-tutorial
	rm -rf /tmp/book/*
	cp -rp small-rust-tutorial/book/* /tmp/book/
	cd /tmp/book && \
		git add -A && \
		git commit -m "deployed on $(shell date) by ${USER}" && \
		git push origin gh-pages
	git update-ref -d refs/heads/gh-pages
	git push --force

all: format lint test run