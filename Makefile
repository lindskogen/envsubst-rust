build:
	cargo build --release


install: build
	mv target/release/envsubst ~/.bin/.
