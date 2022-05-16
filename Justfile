generate:
		cargo r -q -- --dot > output.dot && dot -Tpng output.dot -o output.png
