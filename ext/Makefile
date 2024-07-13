ifeq '$(findstring ;,$(PATH))' ';'
    OS := Windows
else
    OS := $(shell uname 2>/dev/null || echo Unknown)
    OS := $(patsubst CYGWIN%,Cygwin,$(OS))
    OS := $(patsubst MSYS%,MSYS,$(OS))
    OS := $(patsubst MINGW%,MSYS,$(OS))
endif


ifeq ($(OS),Windows)
    SUFFIX := dll
endif
ifeq ($(OS),Darwin)
    SUFFIX := dylib
endif
ifeq ($(OS),Linux)
    SUFFIX := so
endif
ifeq ($(OS),GNU)
    SUFFIX := so
endif


all:
	cargo build --release

clean:
	rm -rf target

install:
	cp target/release/libext.$(SUFFIX) ./

test:
	cargo test

build-macos-amd64:
	cargo build --release --target=x86_64-apple-darwin
	cp target/x86_64-apple-darwin/release/libext.dylib ./

build-macos-aarch:
	cargo build --release --target=aarch64-apple-darwin
	cp target/aarch64-apple-darwin/release/libext.dylib  ./

build-linux:
	cargo build --release --target=x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/release/libext.so ./

build-windows:
	cargo build --release --target=x86_64-pc-windows-msvc
	cp target/x86_64-pc-windows-msvc/release/libext.dll  ./