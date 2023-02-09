default:
  just --list

generate:
	cargo r -q -- --dot > output.dot && dot -Tpng output.dot -o output.png

run:
	watchexec --restart "cargo r"

check:
	cargo clippy
	cargo fmt --all -- --check

fix:
	cargo clippy --fix --allow-staged --allow-dirty
	cargo fmt