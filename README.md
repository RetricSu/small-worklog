# Small WorkLog

A lightweight desktop app to record and show daily work logs.

Build on [egui](https://www.egui.rs/).

![product-screen-shot](/assets/product-3.png)

## Install

Download the app according to your OS from the release page:

https://github.com/RetricSu/small-worklog/releases

Note: you may need to update `small-worklog.app`'s permission to run for MacOS. You can use `xattr -d com.apple.quarantine <your-path-to-small-worklog.app>` to fix it but please keep in mind this is very dangerous, if you don't trust the distribution, please build from source.

## Build From Source

## Build

```sh
cargo build --release
```

or

```sh
cargo bundle --release
```

For MacOS, the app is located in the `target/release/bundle/osx/small-worklog.app`

## Develop

```sh
cargo run --features check_version
```
