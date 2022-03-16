### Chapter 1: 命令行工具

#### 编译：

```shell
jvm-rust ❯ cargo build -p ch01_cmd --release
```

#### 运行：

```shell
jvm-rust ❯ ./target/release/ch01_cmd -help
Usage: ./target/release/ch01_cmd [-options] class [args...]

jvm-rust ❯ ./target/release/ch01_cmd -version
version 0.0.1
```
