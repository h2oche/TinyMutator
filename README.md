# TinyMutator

This project use nightly feature(rustc_private).

## Usage

1. Get coverage report of existing Rust project by using [tarpaulin](https://github.com/xd009642/tarpaulin)
  - Download tarpaulin

    ```
    git submodule init
    git submodule update
    ```

  - Build docker image for tarpaulin

    ```
    ./build-tarpaulin.sh
    ```

  - Change `PROJECT_HOME` in `docker.sh` and get coverage report of Rust project

    ```
    vim docker.sh # change PROJECT_HOME
    ./docker.sh
    ## in container
    cd TARGET_PROJECT
    cargo tarpaulin --out Json --output-dir TARGET_PATH
    ```
  
  - Download rustfmt, rustc-dev and llvm-tools-preview

    ```
    rustup component add rustfmt
    rustup component add rustc-dev
    rustup component add llvm-tools-preview
    ```
  
2. Run TinyMutator
  - You can run TinyMutator with tarpaulin included.
  
    ```
    cargo run [TARGET_PATH]
    ```

  - But it is recommended to run TinyMutator with coverage report.

    ```
    cargo run [TARGET_PATH] [COVERAGE_REPORT]
    ```