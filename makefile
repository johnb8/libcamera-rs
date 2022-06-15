c: check
chk: check
lint: check
check:
	cargo clippy
	cargo test -- --test-threads=1

f: format
fmt: format
format:
	clang-format -style=file -i libcamera-bridge/*
	cargo fmt
