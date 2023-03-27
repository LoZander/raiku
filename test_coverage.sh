#!/bin/bash

clean () {
    rm *.profraw
    rm *.profdata
}

report () {
    llvm-cov report --use-color --ignore-filename-regex='/.cargo/registry' $objects
}

show () {
    llvm-cov show --use-color --ignore-filename-regex='/.cargo/registry' $objects --show-instantiations --show-line-counts-or-regions --Xdemangler=rustfilt | less -R
}

objects="--instr-profile=raiku.profdata --object target/debug/deps/raiku-8fdcc19740f44acd --object target/debug/deps/raiku-56a01c0338693aad --object target/debug/deps/raiku-262244a53f1e9d91"
RUSTFLAGS="-C instrument-coverage" cargo test --tests
llvm-profdata merge -sparse default_*.profraw -o raiku.profdata

if [ "$1" = "report" ]; then
    report
fi

if [ "$1" = "show" ]; then
    show
fi
    
if [ "$1" = "" ]; then
    report
fi

if [ "$2" != "--keep" ]; then
    clean
fi