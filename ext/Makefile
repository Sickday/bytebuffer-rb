all:
	cargo build --release

clean:
	rm -rf target

install:
	cp target/release/libext.so ./
	cp target/release/libext.dylib ./
	cp target/release/libext.dll ./

build-release-macos:
	cargo build --release
	cp target/release/libext.dylib ./

build-release-linux:
	cargo build --release
	cp target/release/libext.so ./