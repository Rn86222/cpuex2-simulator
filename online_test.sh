#!/bin/bash

trap 'func' 1 2 3 15

minrt='minrt_mini'

function error() {
    status=$?
    cd ..
    echo "Exit status: $status"
    exit $status
}

trap 'error' 1 2 3 15
rm ./$minrt.bin ./$minrt.data ./$minrt.ppm > /dev/null 2>&1

echo -n "Compiling '$minrt.ml'... "
cd cpuex-2-2023
./to_riscv
make
rm ./test/$minrt.s > /dev/null 2>&1
./min-caml ./test/$minrt
mv ./test/$minrt.s ../$minrt.s
cd ..
echo "done."

echo -n "Assembling '$minrt.s'... "
cd cpuex2-assembler
rm ./$minrt.s ./$minrt.bin ./$minrt.data > /dev/null 2>&1
mv ../$minrt.s ./$minrt.s
cargo run --release -- --file $minrt.s --style bin > /dev/null 2>&1
mv ./$minrt.bin ../$minrt.bin
mv ./$minrt.data ../$minrt.data
cd ..
echo "done."

echo -n "Simulating '$minrt.bin'... "
ulimit -s unlimited && cargo run --release -- --bin $minrt.bin > /dev/null 2>&1
echo "done."

echo "Checking '$minrt.ppm'..."
diff $minrt.ppm $minrt'_ans.ppm' > diff.txt
if [ -s diff.txt ]; then
    echo "Failed"
else
    echo "Success"
fi
