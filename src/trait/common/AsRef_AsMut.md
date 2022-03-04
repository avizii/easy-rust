# `AsRef` trait
```rust
pub trait AsRef<T> 
where
    T: ?Sized, 
{
    fn as_ref(&self) -> &T;
}
```
`AsRef` 用于引用类型到引用类型的转换，针对的是不可变引用 `&T`。

当我们需要获取结构体中某个字段的引用，我们可以实现 `AsRef`，不用 `Borrow` 引用整个结构体。

具体使用如下代码所示：

```rust
fn main() {
    // str
    let s = "Rustacean";
    let _s1: &str = s.as_ref(); // &str -> &str
    let _s2: &[u8] = s.as_ref(); // &str -> &[u8]

    // String
    let s = "Rustacean".to_string();
    let _s1: &str = s.as_ref();  // String -> &str
    let _s2: &[u8] = s.as_ref(); // String -> &[u8]

    // [T]
    let s: &[u32] = &[1, 2, 3]; 
    let _sr: &[u32] = s.as_ref(); // &[u32] -> &[u32]

    // Vec<T>
    let v = vec![1, 2, 3];
    let _vr1: &Vec<i32> = v.as_ref(); // Vec<i32> -> &Vec<i32>
    let _vr2: &[i32] = v.as_ref();  // Vec<i32> -> &[i32]

    // 自定义结构
    struct AsT {
        num: u8,
    }

    // 转换为结构体内的 num 引用
    impl AsRef<u8> for AsT {
        fn as_ref(&self) -> &u8 {
            &self.num
        }
    }

    let a = AsT {
        num: 8
    };

    let _ar: &u8 = a.as_ref(); // AsT -> &u8
}
```

# `AsMut` trait
`AsMut` 跟 `AsRef` 一样，也是用于引用类型到引用类型的转换。不同的是，`AsMut` 针对的是可变引用 `&mut T`。

具体使用如下代码所示：

```rust
fn main() {
    let mut s = "Rustacean".to_string();
    let _s: &mut str = s.as_mut();

    let mut a: [u32; 3] = [1, 2, 3];
    let _a: &mut [u32] = a.as_mut();

    let mut v: Vec<i32> = vec![1, 2, 3];
    let _vr1: &mut Vec<i32> = v.as_mut();
    let _vr2: &mut [i32] = v.as_mut();

    #[derive(Debug)]
    struct AsT {
        num: u8,
    }

    impl AsMut<u8> for AsT {
        fn as_mut(&mut self) -> &mut u8 {
            &mut self.num
        }
    }

    let mut a = AsT {
        num: 8
    };

    let num: &mut u8 = a.as_mut();
    *num = 10;

    println!("{:?}", a);
}
```