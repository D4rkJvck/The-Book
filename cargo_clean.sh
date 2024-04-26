#!/bin/bash

clean_dir() {
    local dir="$1"

    echo "${dir%*/}:"
    # '%*/': skip trailing '/'

    cd "${dir}" || return

    cargo clean

    cd ..
    echo
}

# Store All Directories
# from List Command Output
all_dir=($(ls -d */))

for dir in "${all_dir[@]}"; do
    clean_dir "${dir}"
done
