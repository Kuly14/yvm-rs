# Ylem Compiler Version Manager

Fork of [svm-rs](https://github.com/alloy-rs/svm-rs), to work with our native Ylem compiler.

### Install

```sh
cargo install --locked --git https://github.com/core-coin/yvm-rs
```

### Usage

-   List available versions

```sh
yvm list
```

-   Install a version

```sh
yvm install <version>
```

-   Use an installed version

```sh
yvm use <version>
```

-   Remove an installed version

```sh
yvm remove <version>
```

### TODO

 - [x] Mac Arm Tests
 - [x] Windows Tests
 - [x] Linux Arm Tests
 - [x] Linux x86_64 Tests
 - [x] Mac x86_64 Tests
