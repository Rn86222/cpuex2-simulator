run:
	ulimit -s unlimited && echo "out_fib10.bin" | cargo run --release > result.txt

clean:
	cargo clean
