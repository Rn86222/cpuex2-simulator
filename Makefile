run:
	ulimit -s unlimited && cargo run --release -- --file main.bin > result.txt

clean:
	cargo clean
