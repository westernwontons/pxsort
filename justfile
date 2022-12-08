alias r := run
alias b := build
alias c := clean
alias t := test

default: run

run:
	cargo run

build:
	cargo build

clean:
	cargo clean

test:
	cargo run -- -f images/leaves.jpg -o output/leaves-edited.jpg