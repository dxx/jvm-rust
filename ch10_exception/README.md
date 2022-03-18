### Chapter 10: 异常处理

#### 编译：

```shell
jvm-rust ❯ cargo build -p ch10_exception
```

#### 运行：

```shell
jvm-rust ❯ ./target/debug/ch10_exception -cp "/Users/dxx/Desktop" jvmrust.ch10.ArgsTest Hello World!
[Hello, World!]
```

```shell
jvm-rust ❯ ./target/debug/ch10_exception -cp "/Users/dxx/Desktop" jvmrust.ch10.ArgsTest
java.lang.IndexOutOfBoundsException: No args!
        at jvmrust.ch10.ArgsTest.bar(ArgsTest.java:21)
        at jvmrust.ch10.ArgsTest.foo(ArgsTest.java:13)
        at jvmrust.ch10.ArgsTest.main(ArgsTest.java:8)
```
