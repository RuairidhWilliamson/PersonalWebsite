_default:
	just --list --unsorted

check:
	cat ./contents/posts/*.md | aspell list

build:
	cargo r --release -- build -r contents --minify

zip: build
	ouch compress dist/* rtaw.zip

serve:
	cargo r --release -- serve -r contents --minify --hot-reload

clean:
	rm -r dist
	rm rtaw.zip
	cargo clean
