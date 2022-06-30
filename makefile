c: check
chk: check
lint: check
check:
	cargo clippy
	cargo test -- --test-threads=1
	cppcheck -q libcamera-bridge/*.hpp libcamera-bridge/*.cpp
	clang-tidy --format-style=file libcamera-bridge/*.cpp -- -I/usr/local/include/libcamera -I./target/cxxbridge -I.. --std=c++17

f: format
fmt: format
format:
	clang-format -style=file -i libcamera-bridge/*
	cargo fmt
	clang-tidy --format-style=file --fix --fix-errors --fix-notes libcamera-bridge/*.cpp -- -I/usr/local/include/libcamera -I./target/cxxbridge -I.. --std=c++17
