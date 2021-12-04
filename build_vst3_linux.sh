 #!/bin/bash

cargo build

mkdir -p target/debug/pi.vst3
mkdir -p target/debug/pi.vst3/Contents
mkdir -p target/debug/pi.vst3/Contents/Resources
mkdir -p target/debug/pi.vst3/Contents/x86_64-linux

cp target/debug/libpi_vst3.so target/debug/pi.vst3/Contents/x86_64-linux/pi.so
