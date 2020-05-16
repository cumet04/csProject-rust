#!/bin/sh

# run in container

for target in \
  x86_64-unknown-linux-gnu \
  x86_64-pc-windows-gnu \
;do
  cargo build --release --target $target
done
