# rn

[![Version][version-badge]][changelog]
[![MIT License][license-badge]][license]
[![Rust][rust-badge]][rust]

rn is a command line tool to rename files and directories.

Its behavior is comparable with that of `mv path/to/old_name path/to/new_name`.  
You can already write this quiet elegant like this `mv path/to/{old_name,new_name}`.
But rn has a more intuitive syntax and is arguably more semantic.

## Requirements

### Development

- [Docker][docker]
- [Docker-Compose][docker-compose]

## Installation

### Download binary

[Archives of precompiled binaries for rn are available.][releases]

### Build binary

1. Clone this repository:

    ```shell
    git clone git@github.com:thled/rn.git
    ```

1. Change to project directory:

    ```shell
    cd rn
    ```

1. Build and start the docker containers:

    ```shell
    docker-compose up -d
    ```

1. Build the app:

    ```shell
    docker-compose exec app cargo build --release
    ```

1. Copy binary:

    ```shell
    cp app/target/release/rn TARGET
    ```

## Usage

`$ rn SOURCE NEW_NAME`

- SOURCE = the file/directory to rename (possibly with absolute/relative path)
- NEW\_NAME = new name of the file/directory

`$ rn --help` for further assistance.

### Examples

- `$ rn foo_file bar_file`
- `$ rn path/to/foo_file bar_file`
- `$ rn foo_dir bar_dir`
- `$ rn path/to/foo_dir bar_dir`
- `$ rn /absolute/path/to/foo_file bar_file`
- `$ rn /absolute/path/to/foo_dir bar_dir`

## Developing

### Enter container (required for following commands)

```shell
docker-compose exec app sh
```

### Build

```shell
cargo build
```

### Run

```shell
cargo run foo bar
```

### Lint

```shell
cargo clippy
```

### Fix

```shell
cargo fix
```

### Format

```shell
cargo fmt
```

### Test

```shell
cargo test
```

### Watch

```shell
cargo watch -cx test -i tests/data
```

## Contribute

Please do contribute! Issues and pull requests are welcome.

[version-badge]: https://img.shields.io/badge/version-0.1.0-blue.svg
[changelog]: ./CHANGELOG.md
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license]: ./LICENSE
[rust-badge]: https://img.shields.io/badge/Rust-1.53-blue.svg
[rust]: https://blog.rust-lang.org/2021/06/17/Rust-1.53.0.html
[docker]: https://docs.docker.com/install
[docker-compose]: https://docs.docker.com/compose/install
[releases]: https://github.com/thled/rn/releases
