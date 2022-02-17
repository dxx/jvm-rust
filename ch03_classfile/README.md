### Chapter 3: 解析 Class 文件

#### 编译：

```shell
jvm-rust ❯ cargo build -p ch03_classfile
```

#### 运行：

```shell
jvm-rust ❯ ./target/debug/ch03_classfile java.lang.String
classpath: /Users/dxx/Desktop/jvm-rust class: java.lang.String args: []
version: 52.0
constants count: 548
access flags: 0x31
this class: java/lang/String
super class: java/lang/Object
interfaces: ["java/io/Serializable", "java/lang/Comparable", "java/lang/CharSequence"]
fields count: 5
 value
 hash
 serialVersionUID
 serialPersistentFields
 CASE_INSENSITIVE_ORDER
methods count: 94
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 checkBounds
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 <init>
 length
 isEmpty
 charAt
 codePointAt
 codePointBefore
 codePointCount
 offsetByCodePoints
 getChars
 getChars
 getBytes
 getBytes
 getBytes
 getBytes
 equals
 contentEquals
 nonSyncContentEquals
 contentEquals
 equalsIgnoreCase
 compareTo
 compareToIgnoreCase
 regionMatches
 regionMatches
 startsWith
 startsWith
 endsWith
 hashCode
 indexOf
 indexOf
 indexOfSupplementary
 lastIndexOf
 lastIndexOf
 lastIndexOfSupplementary
 indexOf
 indexOf
 indexOf
 indexOf
 lastIndexOf
 lastIndexOf
 lastIndexOf
 lastIndexOf
 substring
 substring
 subSequence
 concat
 replace
 matches
 contains
 replaceFirst
 replaceAll
 replace
 split
 split
 join
 join
 toLowerCase
 toLowerCase
 toUpperCase
 toUpperCase
 trim
 toString
 toCharArray
 format
 format
 valueOf
 valueOf
 valueOf
 copyValueOf
 copyValueOf
 valueOf
 valueOf
 valueOf
 valueOf
 valueOf
 valueOf
 intern
 compareTo
 <clinit>
```
