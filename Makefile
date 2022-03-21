build:
	cargo build --release


install: build
	cp target/release/envsubst ~/.bin/.
