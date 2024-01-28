alias c := check

_default:
	just --list --unsorted

check:
	cargo fmt --check
	cargo clippy
	cat ./contents/posts/*.md | aspell list --mode=markdown

build:
	cargo r --release -- build -r contents --minify

zip: build
	ouch compress dist/* rtaw.zip

serve:
	cargo r --release -F server -- serve -r contents --minify --hot-reload

clean:
	rm -r dist
	rm rtaw.zip
	cargo clean
