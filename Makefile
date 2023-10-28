run:
	ulimit -s unlimited && echo "main.bin" | cargo run --release > result.txt

clean:
	cargo clean
