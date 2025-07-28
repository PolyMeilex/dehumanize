# Dehumanize CLI

[![version](https://img.shields.io/crates/v/dehumanize.svg)](https://crates.io/crates/dehumanize)

Are you tired of those pesky humans supplying you binary data in human-readable formats?  
Dehumanize can help you translate the data to a format that you can understand (aka. raw bytes).

```sh
echo "FFFF" | dehumanize --hex
echo "FF FF" | dehumanize --hex --separator " "
echo "0xFF, 0xFF" | dehumanize --hex --separator ","
echo "255, 255" | dehumanize --separator ","
```
