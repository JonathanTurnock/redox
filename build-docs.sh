#!/bin/bash

# Combine input files into the output file
cat src/*.lua > examples/lua/globals.lua
cat src/*.d.ts > examples/typescript/globals.d.ts