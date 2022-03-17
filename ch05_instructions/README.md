### Chapter 5: 指令集和解释器

#### 编译：

```shell
jvm-rust ❯ cargo build -p ch05_instructions
```

#### 运行：

GaussTest:

```java
package jvmrust.ch05;

public class GaussTest {

    public static void main(String[] args) {
        int sum = 0;
        for (int i = 1; i <= 100; i++) {
            sum += i;
        }
        System.out.println(sum);
    }

}
```

指定 classpath 运行 jvmrust.ch05.GaussTest :

```shell
jvm-rust ❯ ./target/debug/ch05_instructions -cp "/Users/dxx/Desktop" jvmrust.ch05.GaussTest
...
LocalVars: LocalVars { slots: [Slot { num: 0, _ref: None }, Slot { num: 5050, _ref: None }, Slot { num: 101, _ref: None }] }
OperandStack: OperandStack { size: 0, slots: [Slot { num: 101, _ref: None }, Slot { num: 100, _ref: None }] }
```
