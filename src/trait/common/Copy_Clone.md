# `Copy` trait

```rust
pub trait Copy: Clone { }
```

`Copy` trait 没有任何额外的方法，它只是一个标记 trait。表示值可以进行简单的按位拷贝，该操作也称为浅拷贝。

虽然 `Copy` 没有任何行为，但它可以用作类型约束来进行类型安全检查。

如果某个类型实现了 `Copy`，该类型会由默认的 Move 语义转换为 Copy 语义，

那么在赋值、函数调用的时候，值会被拷贝，原始值的所有权不会被转移。

📷 画图

## 如何实现 `Copy` trait

### 通过派生宏 `#[derive]` 自动实现

如果自定义结构体的所有字段类型都实现了 `Copy`，则可以用宏 `#[derive(Copy, Clone)]` 来为结构体实现 `Copy`：

```rust
#[derive(Copy, Clone)]
struct Student {
    age: u8,
    gender: u8,
}

fn main() {
    let s1 = Student {
        age: 8,
        gender: 1,
    };

    println!("{} {}", s1.age, s1.gender);
  
    let s2 = s1;  // copy 语义

    println!("{} {}", s1.age, s1.gender); // s1的所有权还在 
}
```

上面的实例代码是可运行的，因为 `u8` 类型实现了 `Copy`, 而下面所示的代码就会出错，因为 `String` 类型没有实现 `Copy`:

```rust
// ⚠️
#[derive(Copy, Clone)]
struct Teacher {
    name: String,
    age: u8,
}
```

### 通过 `impl` 手动实现

看上面的定义，`Copy` 继承于 `Clone`，如果要实现 `Copy` 的话，必须实现 `Clone`，然后实现一个空的 `Copy`。

```rust
struct MyStruct;

impl Copy for MyStruct {}

impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}
```

## 标准库中实现 `Copy` 的常用类型

* 原生类型，包括函数、不可变引用和裸指针实现了 `Copy`
* 数组和元组，如果其内部的类型实现了 `Copy`，那么他们也实现了 `Copy`
* 可变引用没有实现 `Copy`
* 非固定大小的数据结构，没有实现 `Copy`

![std_lib_copy.png](./assets/std_lib_copy.png)

## `Copy` trait 的底层实现

或许部分读者会有疑问：既然 `Copy` 是个没有行为的标记 trait，那么按位拷贝的行为究竟是谁干的？拷贝出来的数据该如何回收？

在这里，基于 `x86` CPU，以下面代码为例，进行简单地分析：

```rust
fn main() {
    let a = 100_u32;
    let b = a;
}
```

当代码 `let a = 100_u32` 执行时，`100_u32` 会被写入到 CPU 寄存器或栈上(main 方法栈帧的局部变量区)；

再往下执行 `let b = a`，CPU 通过执行 `MOV` 指令，将 `a` 的值复制到另一个 CPU 寄存器或者栈上；

这个时候，变量 `a` 和 `b` 所指向的数据都是独立存在、互不影响的。

当代码执行完成后，main 方法的栈帧弹出，栈上的数据自动清空；

而寄存器的数据也会在其他方法执行时被替换；这样就完成了内存资源的回收。

因此，我们可以得出结论：**实现 `Copy` 的类型是要能在栈上分配存储空间，然后借助栈的特性，自动实现内存资源回收。**

## 为何不可变引用 `&T` 实现了 `Copy`，而可变引用 `&mut T` 没有实现 `Copy`？

Rust 的所有权规则规定：**同一时刻，在同一作用域下，只能拥有要嘛一个可变引用，要嘛任意多个不可变引用。**

因为可以存在任意多个不可变引用，所以不可变引用 `&T` 可以实现 `Copy`，传参、赋值时进行引用复制；

相反地，如果可变引用 `&mut T` 实现了 `Copy`，那么生成一个可变引用，

然后把它复制给另一个变量时，就会违背 Rust 的所有权规则：**同一个作用域下只能有一个可变引用**。

# `Clone` trait

```rust
pub trait Clone {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) { ... }
}
```

`Clone` 主要用于对象数据拷贝，该操作是**深拷贝**：

若对象数据只存储在栈上，则拷贝栈上数据；

若对象数据一部分存储在栈上，一部分存储在堆上，则该对象的栈内存和堆内存会一起拷贝。

📷 画图

## 方法说明

| 方法名         | 默认实现 | 方法说明                                                      |
| -------------- | -------- | ------------------------------------------------------------- |
| `clone`      | 无       | 返回拷贝后的值，需手动实现                                    |
| `clone_from` | 有       | 从指定源值拷贝，相比 `clone` 方法，可避免内存分配，提高效率 |

示例代码如下：

```rust
fn main() {
    let s: &str = "Rustacean";
    let _sc: &str = s.clone();

    let s: String = "Rustacean".to_string();
    let _sc: String = s.clone();

    #[derive(Clone)]
    struct Reading<T> {
        frequency: T,
    }

    let r: Reading<&str> = Reading { frequency: "String" };
    let _rc: Reading<&str> = r.clone();

    let source = "Rustacean".to_string();
    let mut s = String::new();
    s.clone_from(&source);
}
```

## 如何实现 `Clone` trait

### 通过派生宏 `#[derive]` 自动实现

如果自定义结构体的所有字段类型都实现了 `Clone`，则可以用宏 `#[derive(Clone)]` 来为结构体实现 `Clone`:

```rust
#[derive(Clone)]
struct Student {
    age: u8,
    gender: u8,
}

fn main() {
    let s1 = Student {
        age: 8,
        gender: 1,
    };

    println!("{} {}", s1.age, s1.gender);
  
    let s2 = s1.clone();  // s2 由 s1 拷贝而来，两个是完全独立的对象，存储在不同的内存地址里

    println!("{} {}", s1.age, s1.gender); // s1的所有权还在 
}
```

### 通过 `impl` 手动实现

```rust
struct MyStruct;

impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}
```

## `Clone` trait 的使用说明

当值类型没有实现 `Copy`，在传参、赋值时，值会发生所有权转移；

为了使原始值在传参、赋值之后还可以使用，确实可以使用 `.clone()` 复制出一份可以独立释放的值副本。

不过在实际开发时，建议还是先使用 borrow 语义对值进行借用，不到迫不得已时，不使用 `.clone()` 进行复制。

因为 `Clone` 的拷贝是深拷贝，对于某些存储于堆内存的数据结构而言，这个拷贝操作耗费可能是很昂贵的。

# `Copy` 和 `Clone` 的区别

虽然 `Copy` 继承于 `Clone`，并且两者都是用来做数据拷贝，它们之间还是有一些区别：

|          | `Copy`                         | `Clone`                                        |
| -------- | -------------------------------- | ------------------------------------------------ |
| 拷贝方式 | 浅拷贝-只针对栈内存按位拷贝      | 深拷贝-同时针对栈内存和堆内存拷贝                |
| 使用方式 | 在传参、赋值、函数返回时自动触发 | 手动显式调用 `.clone()`                        |
| 拷贝开销 | 开销小                           | 开销可大可小；数据存储在堆上的数据结构开销会较大 |
