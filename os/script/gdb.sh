#!/bin/bash

# 本脚本需要 wezterm

wezterm cli split-pane --right \
	riscv64-elf-gdb \
	-ex 'file target/riscv64gc-unknown-none-elf/release/os' \
	-ex 'set arch riscv:rv64' \
	-ex 'target remote localhost:1234'

clear
LOG=TRACE make debug
echo
echo "等待gdb进程结束..."
waitpid $(procs | rg riscv64-elf-gdb | choose 0 | head -n1)
