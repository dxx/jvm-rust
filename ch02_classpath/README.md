### Chapter 2: 查找 Class 文件

#### 编译：

```shell
jvm-rust ❯ cargo build -p ch02_classpath
```

#### 运行：

```shell
jvm-rust ❯ ./target/debug/ch02_classpath -cp "/Library/Java/JavaVirtualMachines/jdk1.8.0_181.jdk/Contents/Home/jre" java.lang.Object
classpath: /Library/Java/JavaVirtualMachines/jdk1.8.0_181.jdk/Contents/Home/jre class: java.lang.Object args: []
class data: [202, 254, 186, 190, 0, 0, 0, 52, 0, 78, 7, 0, 49, 10, 0, 1, 0, 50, 10, 0, 17, 0, 51, 10, 0, 52, 0, 53, 10, 0, 1, 0, 54, 8, 0, 55, 10, 0, 17, 0, 56, 10, 0, 57, 0, 58, 10, 0, 1, 0, 59, 7, 0, 60, 8, 0, 61, 10, 0, 10, 0, 62, 3, 0, 15, 66, 63, 8, 0, 63, 10, 0, 17, 0, 64, 10, 0, 17, 0, 65, 7, 0, 66, 1, 0, 6, 60, 105, 110, 105, 116, 62, 1, 0, 3, 40, 41, 86, 1, 0, 4, 67, 111, 100, 101, 1, 0, 15, 76, 105, 110, 101, 78, 117, 109, 98, 101, 114, 84, 97, 98, ...
```
