# `From` & `Into` trait  
`From` 定义如下：
```rust
pub trait From<T>: Sized {
    fn from(_: T) -> Self;
}
```

`Into` 定义如下：
```rust
pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```

两个 trait 是用于值与值之间的转换，要注意的是：**转换会使原始值的所有权发生转移**。

`From` 与 `Into` 是相反的。

当我们实现 `From<T> for U` 时，Rust 自动为我们实现了 `Into<U> for T`。

所以需要的时候，不要去实现 `Into`，只要实现 `From` 就好了。

伪代码模板如下：

```rust
struct S;

struct T;

impl From<S> for T { todo!(); }

// case 1 by from
let s = S;
let t = T::from(s);

// case 2 by into
let s = S;
let t: T = s.into();
```

## 代码示例
```rust
fn main() {
    let s: &str = "Rustacean";
    let _s1: String = String::from(s); // str -> String
    
    let s: &str = "Rustacean";
    let _s2: String = s.into(); // str -> String
}
```