
.PHONY: build

build:
	cargo build --target x86_64-pc-windows-gnu -p radius_client
	cargo build

.PHONY: test

test:
	cargo run --bin radius_client

.PHONY: clean

clean:
	rm -fr target

.PHONY: debug

debug:
	cargo with "cgdb --args {bin} {args}" -- run --bin radius_client
