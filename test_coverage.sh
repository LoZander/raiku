#!/bin/bash

# To run tests with coverage do
# To test with coverage run `test_coverage.sh` with either 
#      - report (for procentage)
#      - or show (for details)
# The coverage files are removed afterwards, but can be kept with optional argument --keep.
# Note: the `objects` (as defined in the script) correspond to the src .rs files, and must be
# manually updated if new files are added to the project.

clean () {
    rm *.profraw
    rm *.profdata
}

report () {
    llvm-cov report --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=rust_words.profdata $objects
}

show () {
    llvm-cov show --use-color --ignore-filename-regex='/.cargo/registry' --instr-profile=rust_words.profdata $objects --show-instantiations --show-line-counts-or-regions --Xdemangler=rustfilt | less -R
}

objects="--object target/debug/deps/rust_words-50e1691ac3fe6311 --object target/debug/deps/rust_words-9385d823f8572727 --object target/debug/deps/rust_words-9870031fe5c5a151"
RUSTFLAGS="-C instrument-coverage" cargo test --tests
llvm-profdata merge -sparse default_*.profraw -o rust_words.profdata

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