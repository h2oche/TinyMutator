# cs453-project

## Usage

1. Get coverage report of existing Rust project by using [tarpaulin](https://github.com/xd009642/tarpaulin)
  1. Download tarpaulin
    ```
    git submodule init
    git submodule update
    ```
  1. Build docker image for tarpaulin
    ```
    ./build-tarpaulin.sh
    ```
  1. Change `PROJECT_HOME` in `docker.sh` and get coverage report of Rust project
    ```
    vim docker.sh # change PROJECT_HOME
    ./docker.sh
    ## in docker
    cd TARGET_PROJECT
    cargo tarpaulin --out Json --output-dir TARGET_PATH
    ```