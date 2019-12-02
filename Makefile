ifeq ($(shell uname), Darwin)
TARGET := x86_64-apple-darwin
else
TARGET := x86_64-unknown-linux-gnu
endif

all: build

build:
	cargo build --target $(TARGET)

run:
	cargo run

fmt:
	cargo fmt

clean:
	cargo clean
