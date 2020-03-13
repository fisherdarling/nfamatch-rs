build:
	rustup override set nightly-2020-02-26
	cargo build --bin NFAMATCH;
	@cp ./target/debug/NFAMATCH ./NFAMATCH;
	@chmod +x ./NFAMATCH

clean:
	cargo clean
	rm *.m
	rm *.cmptt
	rm *.tt