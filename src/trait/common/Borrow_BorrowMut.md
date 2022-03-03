# `Borrow` trait

```rust
pub trait Borrow<Borrowed> 
where
    Borrowed: ?Sized, 
{
    fn borrow(&self) -> &Borrowed;
}
```

`Borrow` 是表示数据借用的行为，得到的是不可变引用。

当一个类型 `U` 实现 `Borrow<T>`，则该类型的对象调用 `.borrow()`方法可以得到一个不可变引用`&T`，该行为也称之为借用。

伪代码模板如下：

```rust
impl Borrow<T> for U {
    todo!();
}

let u = U;
let t: &T = u.borrow();
```

## 标准库默认实现

### 泛型 `T` 和 `&T` 借用

Rust 标准库为泛型 `T` 、 `&T` 、 `&mut T` 自动实现了 `Borrow<T>` trait。

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Borrow<T> for T {
    #[rustc_diagnostic_item = "noop_method_borrow"]
    fn borrow(&self) -> &T {
        self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Borrow<T> for &T {
    fn borrow(&self) -> &T {
        &**self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Borrow<T> for &mut T {
    fn borrow(&self) -> &T {
        &**self
    }
}
```

举个例子，我们自定义一个数据结构，Rust 会自动把上面的 `T` 替换成我们定义的数据结构，我们就可以直接调用 `.borrow()` 方法：

```rust
fn main() {
    // 自定义结构体
    struct AutoBorrow {
        num: i8,
    }

    // 创建对象
    let mut a = AutoBorrow {
        num: 8,
    };

    // 调用借用方法
    let _a1: &AutoBorrow = a.borrow();           // impl<T: ?Sized> Borrow<T> for T
    let _a3: &AutoBorrow = &a.borrow();          // impl<T: ?Sized> Borrow<T> for &T
    let _a2: &&AutoBorrow = &a.borrow();         // impl<T: ?Sized> Borrow<T> for T
    let _a4: &AutoBorrow = &mut a.borrow();      // impl<T: ?Sized> Borrow<T> for &mut T
    let _a5: &mut &AutoBorrow = &mut a.borrow(); // impl<T: ?Sized> Borrow<T> for T

    assert_eq!(a.borrow(), &a);
}
```

如上面代码所示，类型 `AutoBorrow` 并没有实现 `Borrow`，但是它的对象及引用都可以调用 `.borrow()` 方法，实际上是 Rust 编译器在替我们”负重前行“。

事实上，Rust 默认实现的 `impl<T: ?Sized> Borrow<T> for T `，其实就是不可变引用 `&T`。

### 字符串 `String` 借用

```rust
impl Borrow<str> for String
```

`String` 不仅能借用为 `&String` ，还可以借用为字符串切片 `str`。

```rust
fn main() {
    // str 
    let s1: &str = "easy rust for Rustacean";
    let _s2: &str = s1.borrow(); // impl<T: ?Sized> Borrow<T> for &T

    // string
    let s1: String = "Rustacean".to_string();
    let _s2: &str = s1.borrow(); // impl Borrow<str> for String
    let _s3: &String = s1.borrow(); // impl Borrow<T> for T
}
```

### 数组 `[T; N]` 及动态数组 `Vec<T>` 借用

```rust
impl<T, const N: usize> Borrow<[T]> for [T; N]

impl<T> Borrow<[T]> for Vec<T, Global>
```

无论静态数组，还是动态数组，都可以通过借用得到切片 `[T]`。

```rust
fn borrow_trait() {
    // array
    let a1: [i32; 4] = [1, 2, 3, 4];
    let a2: &[i32; 4] = a1.borrow();  // impl Borrow<T> for T
    assert_eq!(a2, &a1);
    let _a3: &[i32] = a1.borrow(); // impl Borrow<[T]> for [T; N]

    // vec
    let v1: Vec<i32> = vec![1, 2, 3, 4];
    let v2: &Vec<i32> = v1.borrow(); // impl Borrow<T> for T
    assert_eq!(v2, &v1);
    let _v3: &[i32] = v1.borrow(); // impl Borrow<[T]> for Vec<T>
}
```

### 自定义实现 Borrow

```rust
fn main() {
    // 自定义结构体
    struct CusBor {
        name: String,
    }

    // 借用得到 name 属性的字符串切片
    impl Borrow<str> for CusBor {
        fn borrow(&self) -> &str {
            self.name.as_str()
        }
    }

    // 借用得到 name 属性的不可变引用
    impl Borrow<String> for CusBor {
        fn borrow(&self) -> &String {
            &self.name
        }
    }

    let c1 = CusBor {
        name: "Rust".to_string()
    };
    let _c2: &CusBor = c1.borrow();   // impl Borrow<T> for T
    let _c3: &str = c1.borrow();      // impl Borrow<str> for CusBor
    let _c4: &String = c1.borrow();   // impl Borrow<String> for CusBor
    let _c5: &CusBor = &c1.borrow();  // impl<'_, T> Borrow<T> for &'_ T
}
```

# `BorrowMut` trait

# Borrow 与 AsRef 的区别
