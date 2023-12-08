# cpuex2-simulator
Simulator for CPUEX-Group2 computer
```
Usage: simulator [OPTIONS]

Options:
  -b, --bin <BIN>            Name of the input binary file [default: main.bin]
  -s, --sld <SLD>            Name of sld file for raytracing [default: ./sld/contest.sld]
  -v, --verbose              Verbose mode If this flag is set, the simulator will print the value of registers and state of pipeline in each cycle
  -t, --test-fpu <TEST_FPU>  Operation name for test of FPU (fadd, fsub, fmul, fdiv, fsqrt, flt, fcvtsw, or fcvtws)
  -h, --help                 Print help
  -V, --version              Print version
```