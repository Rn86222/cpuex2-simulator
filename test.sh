#!/bin/bash

trap 'func' 1 2 3 15

minrt='minrt_mini'
dirpath=__test_`cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 16 | head -n 1`

function error() {
    status=$?
    cd ..
    rm -rf $dirpath
    echo "Exit status: $status"
    exit $status
}

trap 'error' 1 2 3 15

mkdir $dirpath
cd $dirpath

echo -n "Compiling '$minrt.ml'... "
git clone https://github.com/utokyo-compiler/cpuex-2-2023.git > /dev/null 2>&1
cd cpuex-2-2023
git checkout rn > /dev/null 2>&1
./to_riscv
make > /dev/null 2>&1
rm test/$minrt.s > /dev/null 2>&1
./min-caml test/$minrt > /dev/null 2>&1
mv test/$minrt.s ../$minrt.s
cd ..
rm -rf cpuex-2-2023
echo "done."

echo -n "Assembling '$minrt.s'... "
git clone https://github.com/Rn86222/cpuex2-assembler.git > /dev/null 2>&1
cd cpuex2-assembler
rm ./$minrt.s ./$minrt.bin ./$minrt.data > /dev/null 2>&1
mv ../$minrt.s ./$minrt.s
cargo run --release -- --file $minrt.s --style bin > /dev/null 2>&1
mv ./$minrt.bin ../$minrt.bin
mv ./$minrt.data ../$minrt.data
cd ..
rm -rf cpuex2-assembler
echo "done."

echo -n "Simulating '$minrt.bin'... "
git clone https://github.com/Rn86222/cpuex2-simulator.git > /dev/null 2>&1
cd cpuex2-simulator
rm ./$minrt.bin ./$minrt.data ./$minrt.ppm > /dev/null 2>&1
mv ../$minrt.bin ./$minrt.bin
mv ../$minrt.data ./$minrt.data
ulimit -s unlimited && cargo run --release -- --bin $minrt.bin > /dev/null 2>&1
echo "done."

echo "Checking '$minrt.ppm'..."
diff $minrt.ppm $minrt'_ans.ppm' > diff.txt
if [ -s diff.txt ]; then
    echo "Failed"
else
    echo "Success"
fi

cd ..
rm -rf cpuex2-simulator
cd ..
rm -rf $dirpath
