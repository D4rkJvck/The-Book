#!/bin/bash

directories=($(ls -d */))

for dir in "${directories[@]}"; do
    cd "${dir}"
    cargo clean
    cd ..
done
