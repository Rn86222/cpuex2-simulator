run:
	ulimit -s unlimited && cargo run --release -- --file main.bin > result.txt

min-rt:
	ulimit -s unlimited && cargo run --release -- --file minrt.bin > minrt-result.txt

clean:
	cargo clean
