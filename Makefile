run:
	ulimit -s unlimited && cargo run --release -- --file main.bin > result.txt

minrt:
	ulimit -s unlimited && cargo run --release -- --file minrt.bin > minrt-result.txt

minrt_mini:
	ulimit -s unlimited && cargo run --release -- --file minrt_mini.bin > minrt_mini-result.txt

clean:
	cargo clean
