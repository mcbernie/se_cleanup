

all: default

windows:
	cargo build --target=x86_64-pc-windows-gnu

default:
	cargo build

toolchain:
	rustup target install x86_64-pc-windows-gnu