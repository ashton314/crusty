.PHONY: cargo
cargo:
	cargo build

a.out: cargo foo.c rusty.h
	gcc foo.c rusty.h libcrusty.dylib
