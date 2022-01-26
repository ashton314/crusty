.PHONY: cargo
cargo:
	cargo build

a.out: cargo foo.c rusty.h cheap_job.h
	gcc foo.c rusty.h cheap_job.h libcrusty.dylib
