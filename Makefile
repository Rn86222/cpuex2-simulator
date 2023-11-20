run:
	ulimit -s unlimited && cargo run --release -- --filef main.bin > result.txt

clean:
	cargo clean
