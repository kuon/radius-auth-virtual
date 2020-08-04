
.PHONY: build

build: linux windows

.PHONY: windows

windows:
	cargo build --target x86_64-pc-windows-gnu -p radius_client

.PHONY: linux

linux:
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


.PHONY: genpatch

genpatch:
	git submodule --quiet foreach --recursive \
		'export NAME="$${PWD##*/}"; git --no-pager diff \
		--src-prefix="a/$${NAME}/" --dst-prefix="b/$${NAME}/"' \
		> submodules.patch

.PHONY: applypatch

applypatch:
	git submodule --quiet foreach --recursive \
		'git checkout .'
	git apply submodules.patch

