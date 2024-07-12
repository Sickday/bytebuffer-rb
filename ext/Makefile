ifeq '$(findstring ;,$(PATH))' ';'
    detected_OS := Windows
else
    detected_OS := $(shell uname 2>/dev/null || echo Unknown)
    detected_OS := $(patsubst CYGWIN%,Cygwin,$(detected_OS))
    detected_OS := $(patsubst MSYS%,MSYS,$(detected_OS))
    detected_OS := $(patsubst MINGW%,MSYS,$(detected_OS))
endif


ifeq ($(detected_OS),Windows)
    SUFFIX := dll
endif
ifeq ($(detected_OS),Darwin)        # Mac OS X
    SUFFIX := dylib
endif
ifeq ($(detected_OS),Linux)
    SUFFIX := so
endif
ifeq ($(detected_OS),GNU)           # Debian GNU Hurd
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

build-release-macos:
	cargo build --release
	cp target/release/libext.dylib ./

build-release-linux:
	cargo build --release
	cp target/release/libext.so ./