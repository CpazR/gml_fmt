#!/bin/bash
cargo run -- -f benches/samples/small_test.gml > ignored/output.yaml; 
code ignored/output.yaml;