#!/bin/bash
# Build and run codegen script
# Usage: ./build-and-codegen.sh

echo "Building the project..."
cargo build
cargo run -- --codegen

export PATH=${HOME}/.cargo/bin:${PATH}
reflectapi codegen --language typescript --format --output client
echo "Typescript generation completed"
