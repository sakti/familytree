generate:
		cargo r -q -- --dot > output.dot && dot -Tpng output.dot -o output.png

run:
	watchexec --restart "cargo r"

check:
	cargo clippy

fix:
	cargo clippy --fix --allow-staged