# E2EE SDK

## Install pre-requisites
1. Install build system packages
	```bash
	$ sudo apt install -y rustc cargo
	```
2. System packages for cross-compile base on your target architect.
	```bash
    $ rustup target add x86_64-pc-windows-msvc
	$ rustup target add aarch64-apple-ios x86_64-apple-ios
    $ rustup target add aarch64-linux-android x86_64-linux-android
	```

3. Local packages
	None

## Build E2EE SDK
1. Clone this repo to local and cd to the cloned repo.

2. Building E2EE SDK library, you can cross-compile base on your target architect.
	```bash
	$ cargo rustc --crate-type=cdylib
    $ cargo build --target x86_64-pc-windows-msvc
    $ cargo build --target aarch64-apple-ios
    $ cargo build --target x86_64-apple-ios
    $ cargo build --target aarch64-linux-android
    $ cargo build --target x86_64-linux-android
	```
Note: Because cargo currently doesn't support build multiple architect at the same time, so "cargo test" cannot work in case --crate-type=cdylib since the tests cannot link to cdylib (only supported lib, rlib). So add crate-type = ["cdylib"] or whatever you want to cross-compile as your will.

## Run & Test
1. Clone this repo to local and cd to the cloned repo.

2. Run test with the --nocapture flag will disable output capturing and allow you to see the println! output directly
    ```bash
	$ cargo test -- --nocapture
	```

## Integrity
1. Include to your project