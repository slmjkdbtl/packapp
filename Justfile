# wengwengweng

name := "packapp"

run +args="":
	cargo run -- {{args}}

build:
	cargo build --release

install:
	cargo install \
		--force \
		--path .

pack:
	rm -rf dist
	mkdir -p dist
	cp target/release/{{name}} dist/{{name}}
	upx dist/{{name}}
	zip dist/{{name}}-x86_64-apple-darwin.zip dist/{{name}}
	rm dist/{{name}}
	sha256sum dist/{{name}}-x86_64-apple-darwin.zip

doc crate="packapp":
	cargo doc \
		--no-deps \
		--open \
		--all-features \
		-p {{crate}}

update:
	cargo update
	cargo outdated --root-deps-only

bloat:
	cargo bloat --release --crates

loc:
	loc

