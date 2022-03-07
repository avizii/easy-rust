# `FromStr` trait
```rust
pub trait FromStr {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}
```

`FromStr` 用来将字符串切片 `str` 转换成别的值。

关联类型 `Err` 用来表示转换过程中出现的错误。

实际使用中，更多是调用 `.parse()` 方法，该方法的实现如下：
```rust
#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
    FromStr::from_str(self)
}
```
可以看到，`parse` 是调用 `FromStr::from_str` 实现的。

`FromStr` 使用的伪代码模板如下：
```rust
struct T;

impl FromStr for T {
    todo!();
}

let s: &str = "Rustacean";

let t: T = s.parse().unwrap();
let t = s.parse::<T>().unwrap();
```
## 特别说明
1.因为 `FromStr` 没有生命周期参数，所以只能转换成不包含生命周期的类型。

比如，我们可以用 `FromStr` 转换成 `i32`，但不能转换成 `&i32`：
```rust
// ✅
impl FromStr for i32 {}

// ❌
impl FromStr for &i32 {}
```

再比如，可以用 `FromStr` 转换成带有 `i32` 参数的结构体，但不能转换成带有 `&i32` 参数的结构体：
```rust
// ✅
struct Wow1(i32);
impl FromStr for Wow1 {}

// ❌
struct Wow2(&i32);
impl FromStr for Wow2 {}
```
2.调用 `.parse()` 时，因为标准库提供了很多 `FromStr` 默认实现，所以调用的语句需要显式标注返回类型。

显式类型标注有 2 种方式，见如下代码。
```rust
// case 1 with let assign type
let t: T = s.parse().unwrap();

// case 2 with generic turbofish
let t = s.parse::<T>().unwrap();
```

## 代码示例
```rust
use std::str::FromStr;

fn main() {
    let s: &str = "5";
    
    let _fs: i32 = i32::from_str(s).unwrap();
    
    let _fs: i32 = s.parse().unwrap();
    let _fs = s.parse::<i32>().unwrap();
}
```