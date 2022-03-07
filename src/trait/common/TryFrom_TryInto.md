# `TryFrom` & `TryInto` trait
`TryFrom` 定义如下：
```rust
pub trait TryFrom<T> {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
```

`TryInto` 定义如下：
```rust
pub trait TryInto<T> {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

两个 trait 是用于值与值之间的转换，要注意的是：**转换会使原始值的所有权发生转移**。

`TryFrom` 与 `TryInto` 是相反的。

当我们实现 `TryFom<T> for U` 时，Rust 自动为我们实现了 `TryInto<U> for T`。

所以需要的时候，不要去实现 `TryInto`，只要实现 `TryFom` 就好了。

伪代码模板如下：

```rust
struct S;

struct T;

impl TryFom<S> for T { todo!(); }

// case 1 by try_from and handle error by unwrap
let s = S;
let t = T::try_from(s).unwrap();

// case 2 by try_into and handle error by unwrap
let s = S;
let t: T = s.try_into().unwrap();
```

## 关联类型 `type Error`
上面的说明可以看出，`TryFrom` / `TryInto` 的用法和 `From` / `Into` 一样.

只是 trait 定义内多了关联类型 `Error`，且返回的结果是 `Result<T, Error>`, 用于处理转换过程的错误。

如果你的数据类型在转换过程中有可能出现错误，建议使用 `TryFrom` 和 `TryInto` 进行转换。

## 代码示例
```rust
fn main() {
    // impl TryFrom<i32> for u64
    let _num: i32 = i32::try_from(20_u64).unwrap();
    let _num: i32 = 20_u64.try_into().unwrap();
}
```