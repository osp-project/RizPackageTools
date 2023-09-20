@echo off
if not exist build mkdir build
cargo make build -release --target-dir build --profile static-link
echo Build Successful