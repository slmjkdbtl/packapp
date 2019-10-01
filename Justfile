# wengwengweng

run +args="":
	cargo run -- {{args}}

doc crate="packapp":
	cargo doc --no-deps --open -p {{crate}}

update:
	cargo update

bloat:
	cargo bloat --release --crates

loc:
	loc

checkdep:
	cargo outdated --root-deps-only

