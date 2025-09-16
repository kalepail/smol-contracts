default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	@ls -l ../../target/wasm32v1-none/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean

snapshot:
	stellar snapshot create --network testnet --address GCEDG23LK46PHGXIY63E3ELQGBX6VHQ4EWLYT7FMLOOCIS3ZY2ITHDXB --address GCHPTWXMT3HYF4RLZHWBNRF4MPXLTJ76ISHMSYIWCCDXWUYOQG5MR2AB --output json