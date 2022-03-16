### Chapter 9: 本地方法调用

#### 编译：

```shell
jvm-rust ❯ cargo build -p ch09_native --release
```

#### 运行：

```shell
jvm-rust ❯ ./target/release/ch09_native -cp "/Users/dxx/Desktop" jvmrust.ch09.GetClassTest
void
boolean
byte
char
short
int
long
float
double
java.lang.Object
jvmrust.ch09.GetClassTest
[I
[[I
[Ljava.lang.Object;
[[Ljava.lang.Object;
java.lang.Runnable
java.lang.String
[D
[Ljava.lang.String;
```

```shell
jvm-rust ❯ ./target/release/ch09_native -cp "/Users/dxx/Desktop" jvmrust.ch09.StringTest
true
false
true
```

```shell
jvm-rust ❯ ./target/release/ch09_native -cp "/Users/dxx/Desktop" jvmrust.ch09.ObjectTest
22415000
jvmrust.ch09.ObjectTest@1560698
false
true
```