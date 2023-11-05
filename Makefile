run:
	ulimit -s unlimited && echo "main.bin" | cargo run --release > result2.txt

clean:
	cargo clean
