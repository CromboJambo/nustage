#!/bin/bash

echo "Building GL Tie-Out Rust Project..."
echo ""

cd "$(dirname "$0")"

if [ ! -d "data" ]; then
    echo "ERROR: data directory not found!"
    echo "Please ensure the data directory with CSV files exists."
    exit 1
fi

cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "Build successful!"
    echo "The executable will be at: target/release/gl_tie_out"
    echo ""
    echo "To run the program:"
    echo "  ./target/release/gl_tie_out"
else
    echo ""
    echo "Build failed. Please check the error messages above."
    exit 1
fi
