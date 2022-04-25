# jvm-rust

参考[自己动手写 Java 虚拟机](https://github.com/zxh0/jvmgo-book)源码，用 Rust 语言实现 JVM。

## 开发环境

* MacOS Monterey 12.1
* Rust 1.57.0
* JDK 1.8

## 目录说明

本源码目录结构按照 [jvmgo-book](https://github.com/zxh0/jvmgo-book) 组织，说明如下：

* ch01_cmd: 命令行处理
* ch02_classpath: 查找 Class 文件
* ch03_classfile: 解析 Class 文件
* ch04_rtda: 运行时数据区
* ch05_instructions: 指令集和解释器
* ch06_rtda_heap: 类和对象
* ch07_method_invoke: 方法调用和返回
* ch08_array_string: 数组和字符串
* ch09_native: 本地方法调用
* ch10_exception: 异常处理

`java` 目录存放测试使用的 Java 代码。

## 运行

参考每个目录中的 `README.md`。
