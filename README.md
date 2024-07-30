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
	$ cargo install cargo-ndk
	```

3. Local packages
	None

## Build E2EE SDK
1. Clone this repo to local and cd to the cloned repo.

2. Building E2EE SDK library, you can cross-compile base on your target architect.
	```bash
	$ cargo build
    $ cargo build --target x86_64-pc-windows-msvc
    $ cargo build --target aarch64-apple-ios
    $ cargo build --target x86_64-apple-ios
    $ cargo build --target aarch64-linux-android
    $ cargo build --target x86_64-linux-android
	$ cargo rustc --crate-type=cdylib
	$ cargo ndk --target aarch64-linux-android -- cargo build
	```
Note: Because cargo currently doesn't support build multiple architect at the same time, so "cargo test" cannot work in case --crate-type=cdylib since the tests cannot link to cdylib (only supported lib, rlib). So add crate-type = ["cdylib"] or crate-type = ["dylib"] whatever you want to cross-compile as your will.

## Run & Test
1. Clone this repo to local and cd to the cloned repo.

2. Run test with the --nocapture flag will disable output capturing and allow you to see the println! output directly
For C wrapper test, you need to make sure gcc available, if not install it.
    ```bash
	$ cargo build
	$ cargo rustc --crate-type=cdylib
	$ cargo test -- --nocapture --test-threads=1
	```

3. Manual run test if any wrong.
	```bash
	$ gcc tests/c_wrapper_test.c -o c_wrapper_test -Ltarget/debug -le2esdk -Iinclude
	$ LD_LIBRARY_PATH=./target/debug ./c_wrapper_test
	```

4. Note: For Window platform, you need to install MinGW-w64 gcc to run the tests.

## Deployment
From e2esdk folder run 1 of these commands base on your target architect:
1. Linux:
	```bash
	$ sudo ./deploy/install_sdk_linux.sh
	```

2. Window, run with admin priviledge
	```bash
	$ sudo ./deploy/install_sdk_window.sh
	```

3. Android, add your project folder to script and run
	```bash
	$ sudo ./deploy/install_sdk_android.sh /path/to/your/android-ndk
	```
Then you can use e2e shared library for your android project, point to shared library libe2esdk.so manually if not work.

4. iOS, I expect you rn it in MacOS
	```bash
	$ sudo ./deploy/install_sdk_ios.sh
	```
Then you can use e2e shared library for your ios project, point to shared library libe2esdk.so manually if not work.

## Continous Integration
Build/Run for Linux/MacOS/Window platform, check in Actions tab at github