#!/bin/bash

# Function to display usage
usage() {
    echo "Usage: $0 [output_file] [input_files...]"
    echo "Example: $0 example/rustful.lua src/*.lua"
    exit 1
}

# Check if at least two arguments are provided
if [ "$#" -lt 2 ]; then
    usage
fi

# Get the output file name
output_file="$1"
shift

# Combine input files into the output file
cat "$@" > "$output_file"

# Notify the user
echo "Files combined into $output_file successfully."
