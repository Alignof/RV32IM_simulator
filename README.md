# carron
[![Rust](https://github.com/Alignof/carron/actions/workflows/rust.yml/badge.svg)](https://github.com/Alignof/Carron/actions/workflows/rust.yml)
[![rv32_tests](https://github.com/Alignof/carron/actions/workflows/rv32_tests.yml/badge.svg)](https://github.com/Alignof/Carron/actions/workflows/rv32_tests.yml)
RV32IAMC emulator in rust

## Feature
This emulator parses an ELF file that supports rv32imc and emulate execution environment of it.  
The emulator passed riscv-tests shown below.

- [x] rv32ui-p
- [x] rv32ui-v
- [x] rv32uc-p
- [x] rv32uc-v
- [ ] rv32ua-p
- [ ] rv32ua-v

## Install
```zsh
git clone https://github.com/Alignof/carron.git
cd carron
cargo build --release
```

## Usage
```zsh
$ ./carron --help
carron 0.9.2
n.takana <Alignof@outlook.com>
RV32IMAC emulator

USAGE:
    carron [OPTIONS] <filename>

ARGS:
    <filename>    ELF file path

OPTIONS:
    -e, --elfhead                         Show ELF header
    -p, --program                         Show all segments
    -s, --section                         Show all sections
    -d, --disasem                         Disassemble ELF
    -a, --all                             Show all ELF data
        --pk <proxy_kernel>               Run with proxy kernel
        --pc <init_pc>                    Set entry address as hex
        --break_point <address>           Set break point as hex
        --result_reg <register_number>    Set result register
    -h, --help                            Print help information
    -V, --version                         Print version information
```
