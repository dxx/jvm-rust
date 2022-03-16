### Chapter 6: 类和对象

#### 编译：

```shell
jvm-rust ❯ cargo build -p ch06_rtda_heap --release
```

#### 运行：

```shell
jvm-rust ❯ ./target/release/ch06_rtda_heap -cp "/Users/dxx/Desktop" jvmrust.ch06.MyObject
...
[Loaded java/lang/Object
[Loaded jvmrust.ch06.MyObject
pc: 0, inst:LDC { index: 2 }
pc: 2, inst:ISTORE_1
pc: 3, inst:NEW { index: 3 }
pc: 6, inst:DUP
pc: 7, inst:INVOKE_SPECIAL { index: 4 }
pc: 10, inst:ASTORE_2
pc: 11, inst:ILOAD_1
pc: 12, inst:PUT_STATIC { index: 5 }
pc: 15, inst:GET_STATIC { index: 5 }
pc: 18, inst:ISTORE_1
pc: 19, inst:ALOAD_2
pc: 20, inst:ILOAD_1
pc: 21, inst:PUT_FIELD { index: 6 }
pc: 24, inst:ALOAD_2
pc: 25, inst:GET_FIELD { index: 6 }
pc: 28, inst:ISTORE_1
pc: 29, inst:ALOAD_2
pc: 30, inst:ASTORE_3
pc: 31, inst:ALOAD_3
pc: 32, inst:INSTANCE_OF { index: 3 }
pc: 35, inst:IFEQ { offset: 18 }
pc: 38, inst:ALOAD_3
pc: 39, inst:CHECK_CAST { index: 3 }
pc: 42, inst:ASTORE_2
pc: 43, inst:GET_STATIC { index: 7 }
[Loaded java/lang/System
pc: 46, inst:ALOAD_2
pc: 47, inst:GET_FIELD { index: 6 }
pc: 50, inst:INVOKE_VIRTUAL { index: 8 }
32768
```
