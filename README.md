# Ylem Compiler Version Manager

Fork of [svm-rs](https://github.com/alloy-rs/svm-rs), to work with our native Ylem compiler.

### Install

```sh
cargo install --locked --git https://github.com/kuly14/yvm-rs
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
 - [ ] Windows Tests
 - [ ] Linux Arm Tests
 - [ ] Linux x86_64 Tests
 - [ ] Mac x86_64 Tests
