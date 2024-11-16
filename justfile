alias c := check
alias b := build

_default:
	just --list --unsorted

check:
	cargo fmt --check
	cargo clippy

build:
	cargo r --release -F progress -- build -r contents --minify

zip: build
	ouch compress dist/* rtaw.zip

serve:
	cargo r --release -F server -F progress -- serve -r contents --minify --hot-reload --grammar-check

clean:
	rm -r dist
	rm rtaw.zip
	cargo clean
