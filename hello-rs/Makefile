build:
	cargo build

test: build
	cargo test

fuzz:
	cargo +nightly fuzz run fuzz_hello

clean:
	cargo clean
	cd fuzz; $(MAKE) clean