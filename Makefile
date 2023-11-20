run:
	ulimit -s unlimited && cargo run --release -- main.bin > result.txt

clean:
	cargo clean
