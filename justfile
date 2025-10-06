# generate documentation
doc:
	cargo run -- -i doc/source.md -o README.md -d "[[[ ]]]"
	pandoc README.md -so doc/index.html -d readme -d pandoc.yml
	pandoc CHANGELOG.md -so doc/changelog/index.html -d readme -d pandoc.yml
