#!/bin/bash

cargo build --release
cp target/release/pass-gen /Applications/utils/passgen
