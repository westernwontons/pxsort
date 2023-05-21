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
	cargo run -- images/leaves.jpg output/leaves-edited.jpg

test-release:
	cargo run --release -- images/leaves.jpg output/leaves-edited.jpg