# 闭包 `Closure`

https://mcll.top/2020/03/09/rust-closure/

https://juejin.cn/post/6991378217701474334

https://blog.linyinfeng.com/posts/how-do-rust-closures-work/

https://m.yisu.com/zixun/542947.html

https://rustcc.cn/article?id=cd3afc4e-58f5-41fd-8a91-cd4accb78b92

https://course.rs/advance/functional-programing/closure.html

https://dhghomon.github.io/easy_rust/Chapter_37.html

https://dhghomon.github.io/easy_rust/Chapter_47.html


闭包：
将函数，或者代码和其环境一起存储的一种数据结构。闭包引用的上下文中的自由变量，会被捕获到闭包的结构中，成为闭包类型的一部分

闭包是一种匿名类型，一旦声明，就会产生一个新的类型，但是这个类型无法被其他地方使用。

这个类型就像一个结构体，会包含所有捕获的变量。（特殊的结构体）

不带move时，闭包捕获的是对应自由变量的引用；
带move时，对应自由变量的所有权会被移动到闭包结构中。

如果不使用 move 转移所有权，闭包会引用上下文中的变量，这个引用受借用规则的约束，所以只要编译通过，那么闭包对变量的引用就不会超过变量的生命周期，没有内存安全问题。

如果使用 move 转移所有权，上下文中的变量在转移后就无法访问，闭包完全接管这些变量，它们的生命周期和闭包一致，所以也不会有内存安全问题。

闭包的大小跟参数、局部变量都无关，只跟捕获的变量有关

闭包是存储在栈上的，并且除了捕获的数据外，闭包本身不包含任何额外函数指针指向闭包的代码

Rust 为每个闭包生成一个新的类型，又使得调用闭包时可以直接和代码对应，省去了使用函数指针再转一道手的额外消耗。

为什么大多数编程语言闭包的性能要远低于函数调用。因为使用闭包就意味着：
额外的堆内存分配、
潜在的动态分派（很多语言会把闭包处理成函数指针）、
额外的内存回收。

在声明闭包的时候，我们并不需要指定闭包要满足的约束，但是当闭包作为函数的参数或者数据结构的一个域时，我们需要告诉调用者，对闭包的约束。

FnOnce：只能被调用一次。

FnMut:
可以被多次调用
想改变闭包捕获的数据结构，那么就需要 FnMut

Fn：
任何需要 FnOnce 或者 FnMut 的场合，都可以传入满足 Fn 的闭包

Rust 支持三种不同的闭包 trait：FnOnce、FnMut 和 Fn。
FnOnce 是 FnMut 的 super trait，而 FnMut 又是 Fn 的 super trait。
从这些 trait 的接口可以看出，
FnOnce 只能调用一次；
FnMut 允许在执行时修改闭包的内部数据，可以执行多次；
Fn 不允许修改闭包的内部数据，也可以执行多次

## 基本语法



### `move`

## 实现原理

## FnOnce

## FnMut

## Fn

## 函数指针 `fn` 与 `FnOnce` / `FnMut` / `Fn` 的关系
`FnOnce` / `FnMut` / `Fn` 是trait，是 `DST`；`fn`是类型，函数指针，它的大小在编译时就已知。

`fn` 也实现了这些 trait ，因此声明参数的时候，我们可以统一使用泛型加特征约束 `Fn`，这样调用的时候，无论传闭包还是传函数指针都是可以。

不过，比如与 c 语言进行交互，就需要用函数指针了。

## 闭包作为函数入参

闭包本质上就是一个重载了call运算符的匿名结构体，不同的闭包，其结构体是不一样的，因此是 `DST`。

当需要使用闭包作为参数时，有 3 种方法可以表示：
1. 使用 `impl Fn(T) -> (U)`
```rust
// FnOnce
fn call_fn_once_1(arg: u64, f: impl FnOnce(u64) -> u64) -> u64 { f(arg) }

// FnMut
fn call_fn_mut_1(arg: u64, f: &mut impl FnMut(u64) -> u64) -> u64 { f(arg) }

// Fn
fn call_fn_1(arg: u64, f: &impl Fn(u64) -> u64) -> u64 { f(arg) }

```
2. 使用 trait object
```rust
// FnOnce
fn call_fn_once_2(arg: u64, f: Box<dyn FnOnce(u64) -> u64>) -> u64 { f(arg) }

// FnMut
fn call_fn_mut_2(arg: u64, mut f: Box<&mut dyn FnMut(u64) -> u64>) -> u64 { f(arg) }

// Fn
fn call_fn_2(arg: u64, f: Box<&dyn Fn(u64) -> u64>) -> u64 { f(arg) }
```

3. 使用泛型
```rust
// FnOnce
fn call_fn_once_3<F>(arg: u64, f: F) -> u64
    where
        F: FnOnce(u64) -> u64,
{ f(arg) }

// FnMut
fn call_fn_mut_3<F>(arg: u64, f: &mut F) -> u64
    where
        F: FnMut(u64) -> u64, 
{ f(arg) }

// Fn
fn call_fn_3<F>(arg: u64, f: &F) -> u64
    where
        F: Fn(u64) -> u64,
{ f(arg) }
```

实际调用时，需注意：
1. 对于 `FnOnce`，可直接传入闭包
2. 对于 `FnMut`，闭包需声明为 `mut`，且函数传入的闭包参数需使用 `&mut`
3. 对于 `Fn`，函数传入的闭包参数需使用 `&`

具体见如下代码示例：

```rust
fn main() {
    println!("start to call FnOnce ......");
    let s = String::from("Rustacean");
    let mut f1 = |x: u64| s.len() as u64 * x;
    let mut f2 = |x: u64| s.len() as u64 * x;
    let mut f3 = |x: u64| s.len() as u64 * x;
    println!("call fn_once: {}", call_fn_once_1(1, f1));
    // println!("call fn_once: {}", call_fn_once_2(1, Box::new(f2)));
    println!("call fn_once: {}", call_fn_once_3(1, f3));

    println!("start to call FnMut ......");
    let mut s = String::from("Rust");
    let mut f = |x: u64| s.len() as u64 * x;
    println!("call fn_mut: {}", call_fn_mut_1(1, &mut f));
    // println!("call fn_mut: {}", call_fn_mut_2(1, &mut f));
    println!("call fn_mut: {}", call_fn_mut_3(1, &mut f));

    println!("start to call Fn ......");
    let s = String::from("Closure");
    let f = |x: u64| s.len() as u64 * x;
    println!("call fn: {}", call_fn_1(1, &f));
    // println!("call fn: {}", call_fn_2(1, &f));
    println!("call fn: {}", call_fn_3(1, &f));
}
```

## 闭包作为函数返回值

与闭包作为函数入参一样，当需要返回闭包时，有 2 种方法可以表示：

1. 使用 `impl Fn(T) -> U`
```rust
fn ret_closure(x: i32) -> impl Fn(i32) -> i32 { todo!() }
```

这种返回方式存在一定的局限性：**只能返回同样的类型**

2. 使用 trait object

```rust
fn ret_closure(x: i32) -> Box<dyn Fn(i32) -> i32> { todo!() }
```
## 使用场景及最佳实践

Rust 闭包的使用场景：

1. 在函数的参数中使用闭包

2. 闭包作为函数的返回值

3. 为闭包实现某个 trait，使其也能表现出其他行为，而不仅仅是作为函数被调用

在实践中，深入理解 `FnOnce`，`FnMut` 和 `Fn` 的语义才能作出正确的选择：
1. `Fn`，函数不保有自己的状态

2. `FnMut`，函数可以改变自己的状态

3. `FnOnce`，函数消费自己的状态

也就是说：

需要纯函数的时候，用 `Fn`

需要函数保存内部状态的时候，如伪随机数生成函数，用 `FnMut`

类似于创建线程这样的调用，用 `FnOnce`