# `ToOwned` trait

```rust
pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;

    fn clone_into(&self, target: &mut Self::Owned) { ... }
}
```

`ToOwned` 与 `Clone` 一样，也是用于对象数据拷贝。不同的是，`Clone` 只适用于从 `&T` 到 `T`。而 `ToOwned` 可以实现从 &U 到 T 的拷贝，只需满足 `impl Borrow<U> for T` 这个条件。

伪代码模板如下：

```rust
impl Borrow<U> for T {}

impl ToOwned for U {
    type Owned = T;
}

let u = &U;

let t: T = u.to_owned();
```

## 关联类型 `type Owned: Borrow<Self>`

`ToOwned` 带有一个关联类型 `Owned`，该类型由 `ToOwned` 的实现方自定义。不过 `Owned` 不能是任意类型，它必须实现 `Borrow<Self>`。

为什么需要满足 `impl Borrow<U> for T` 这个条件呢？

可以这样理解：`impl Borrow<U> for T` 表示 `T` 的不可变引用是 `&U`，反过来看，从引用 `&U` 拷贝数据，得到对象 `T`。

举个例子，用字符串切片 `&str` 或者字符串引用 `&String` 调用 `.to_owned()` 拷贝数据，都能得到拥有所有权的 `String` 对象，因为标准库实现了 `impl Borrow<str> for String`:

```rust
fn main() {
    let s: &str = "Rustacean";
    let _os: String = s.to_owned(); 

    let s: &String = &("Rustacean".to_string());
    let _os: String = s.to_owned(); 
}
```

## 方法说明


| 方法名       | 默认实现 | 方法说明                                                        |
| -------------- | ---------- | ----------------------------------------------------------------- |
| `to_owned`   | 无       | 返回拷贝后的值，需手动实现                                      |
| `clone_info` | 有       | 拷贝到指定目标值，相比`to_owned` 方法，可避免内存分配，提高效率 |

示例代码如下：

```rust
fn main() {
    let s: &str = "Rustacean";
    let _os: String = s.to_owned(); // impl ToOwned for str (type Owned = String)

    let s: &String = &("Rustacean".to_string());
    let _os: String = s.to_owned(); // impl<T> ToOwned for T (type Owned = T)

    let v: &[i32] = &[1, 2, 3];
    let _vo: Vec<i32> = v.to_owned(); // impl<T> ToOwned for [T] (type Owned = Vec<T, Global>)

    let v: &[i32; 3] = &[1, 2, 3];
    let _vo: [i32; 3] = v.to_owned();  // impl<T> ToOwned for T (type Owned = T)

    let v: Vec<i32> = vec![1, 2, 3];
    let _vo: Vec<i32> = (&v).to_owned();
}
```
