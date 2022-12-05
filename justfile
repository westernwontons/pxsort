alias r := run
alias b := build
alias c := clean

default: run

run:
	cargo run

build:
	cargo build

clean:
	cargo clean