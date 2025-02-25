alias c := check
alias b := build

_default:
	just --list --unsorted

check:
	cargo fmt --check
	cargo clippy --workspace

build:
	RUST_LOG=info cargo r --release -- build -r contents --minify --grammar-check

zip: build
	ouch compress dist/* rtaw.zip

serve:
	RUST_LOG=info cargo r --release -F server -- serve -r contents --hot-reload --grammar-check

clean:
	rm -r dist
	rm rtaw.zip
	cargo clean
