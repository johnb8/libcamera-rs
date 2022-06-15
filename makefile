lint:
	cargo clippy

fmt:
	clang-format -style=file -i libcamera-bridge/*
	cargo fmt
