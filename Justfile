default:
  just --list

generate:
	cargo r -q -- --dot > output.dot && dot -Tpng output.dot -o output.png

run:
	watchexec --restart "cargo r"

check:
	cargo clippy
	cargo fmt --all -- --check

fmt:
    rustup run nightly -- rustfmt -v --edition 2021 ./src/main.rs

fix: fmt
	cargo clippy --fix --allow-staged --allow-dirty

meta:
    nix flake metadata

dockerize:
    nix build .#dockerImage
