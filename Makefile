ARCH ?= riscv32
MODE ?= debug

ifeq ($(ARCH), x86_64)
ifeq ($(shell uname), Darwin)
TARGET := x86_64-apple-darwin
else
TARGET := x86_64-unknown-linux-gnu
endif
else ifeq ($(ARCH), aarch64)
TARGET := aarch64-unknown-linux-gnu
else ifeq ($(ARCH), riscv32)
TARGET := riscv32imac-unknown-none-elf
else ifeq ($(ARCH), riscv64)
TARGET := riscv64imac-unknown-none-elf
endif

BUILD_ARGS :=

ifeq ($(MODE), release)
BUILD_ARGS += --release
endif

all: build

env:
	rustup target add $(TARGET)

build:
	cargo build --target $(TARGET) $(BUILD_ARGS)

run:
	cargo run

fmt:
	cargo fmt

clean:
	cargo clean
