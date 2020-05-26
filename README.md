# autotiling

This script uses the [i3ipc Rust library](https://github.com/tmerr/i3ipc-rs) to switch the layout
splith / splitv depending on the currently focused window dimensions.

This script is almost a direct port of https://github.com/nwg-piotr/autotiling from Python to Rust.

## Installation

Run the following commands:
```
$ git clone https://github.com/timothyjin/autotiling
$ cd autotiling
$ cargo build --release
```
The built binary will be in `target/release/autotiling`. Place this file anywhere and autostart it by putting `exec_always --no-startup-id /path/to/the/script/autotiling &` in your i3 config.
