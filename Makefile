run:
	ulimit -s unlimited && echo "out.bin" | cargo run --release > result.txt

clean:
	cargo clean
