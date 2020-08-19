PWD = $(shell pwd)

.PHONY: debug

debug:
	cargo build

.PHONY: release

release: release-host release-windows

.PHONY: release-windows

release-windows:
	cargo build --target x86_64-pc-windows-gnu --release -p auth_client

.PHONY: release-host

release-host:
	cargo build --release

.PHONY: clean

clean:
	rm -fr target
	rm -fr build



.PHONY: test

test: freeradius
	./tests/run.sh

# Run test with memory sanitizer, this requires rust nightly
.PHONY: memory-test

memory-test: export RUSTFLAGS=-Zsanitizer=address
memory-test:
	cargo test -p radius_virtual --target x86_64-unknown-linux-gnu -- --nocapture  --test-threads=1

# Install a mockup freeradius serveur for testing

.PHONY: freeradius

freeradius: kqueue build/freeradius/dist/sbin/radiusd \
	build/freeradius/dist/etc/raddb/certs/rsa/ca.pem

build/freeradius/dist/etc/raddb/certs/rsa/ca.pem:
	./build/freeradius/dist/etc/raddb/certs/bootstrap

build/freeradius/dist/sbin/radiusd: build/freeradius/Make.inc
	cd build/freeradius && make -j 8 && make install

build/freeradius/Make.inc: build/freeradius/configure
	cd build/freeradius && ./configure \
		--with-kqueue-lib-dir=../kqueue/ \
		--with-kqueue-include-dir=../kqueue/include/ \
		--prefix=${PWD}/build/freeradius/dist/

build/freeradius/configure:
	mkdir -p build
	git clone git@github.com:FreeRADIUS/freeradius-server.git build/freeradius

.PHONY: kqueue

kqueue: build/kqueue/libkqueue.so

build/kqueue/libkqueue.so: build/kqueue/CMakeLists.txt
	cd build/kqueue && cmake .
	cd build/kqueue && make

build/kqueue/CMakeLists.txt:
	mkdir -p build
	git clone git@github.com:mheily/libkqueue.git build/kqueue


# Run a cgdb instance of the radius_client
.PHONY:gdb

gdb:
	cargo with "cgdb --args {bin} {args}" -- run --bin radius_client


# Manage patch for submodules
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

