default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	mkdir -p target/wasm32v1-none/optimized
	stellar contract optimize \
		--wasm target/wasm32v1-none/release/smol.wasm \
		--wasm-out target/wasm32v1-none/optimized/smol.wasm
	cd target/wasm32v1-none/optimized/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

fmt:
	cargo fmt --all

clean:
	cargo clean

snapshot:
	stellar snapshot create --network testnet --address GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAGO6V --address CAHERVSQSDMK6ULIG63LQFWD2JOWSRPUMZXQBPPDUPOEV5B5NXESTA2R --address CB6RPCFIL7QAQWLZCAURWUROQ4FY6KRAR7U42UXRTRKXUJRWIHDGPZYK --address GCEDG23LK46PHGXIY63E3ELQGBX6VHQ4EWLYT7FMLOOCIS3ZY2ITHDXB --address GCHPTWXMT3HYF4RLZHWBNRF4MPXLTJ76ISHMSYIWCCDXWUYOQG5MR2AB --output json