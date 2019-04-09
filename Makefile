

current_dir = $(shell pwd)

all: default

windows: windows-toolchain
	env X86_64_PC_WINDOWS_GNU_OPENSSL_LIB_DIR=$(current_dir)/vendor/openssl-1.1.0i-win64-mingw/lib \
		X86_64_PC_WINDOWS_GNU_OPENSSL_INCLUDE_DIR=$(current_dir)/vendor/openssl-1.1.0i-win64-mingw/include \
	cargo build --target=x86_64-pc-windows-gnu --verbose

clean:
	cargo clean

update:
	cargo update

default:
	cargo build

windows-toolchain:
	rustup target install x86_64-pc-windows-gnu