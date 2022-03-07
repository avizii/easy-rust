# `Cow` enum
```rust
pub enum Cow<'a, B> 
where
    B: 'a + ToOwned + ?Sized, 
 {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

`Cow` 是一个用于写时克隆（`clone-on-write`）能力的智能指针。

我们可以用它来包裹一个只读借用，当需要修改借用数据内容或者获得借用数据的所有权时，它会克隆借用的数据，由 `Cow::Borrowed` 转变成 `Cow::Owned`。

这种设计跟操作系统虚拟内容管理的写时复制（`copy-on-write`）有异曲同工之妙。

`Cow` 有个泛型特征约束 `ToOwned`，`ToOwned` 的定义及使用看[这里](../../trait/common/ToOwned.md)。

`Cow` 实现了 `Deref` 来自动解引用：
```rust
impl Deref for Cow<'_, B> { 
    type Target = B; 
    
    fn deref(&self) -> &B { 
        match *self { 
            Borrowed(borrowed) => borrowed, 
            Owned(ref owned) => owned.borrow(), 
        } 
    }
}
```

具体实现是根据 `self` 是 `Borrowed` 还是 `Owned` 分别取其内容，生成引用：

对于 `Borrowed`，直接就是引用；

对于 `Owned`，调用 `.borrow()` 方法，获得引用。


## 方法说明

| 方法名         | 方法说明                                                      |
| -------------- | ------------------------------------------------------------- |
| `to_mut`      | 获取数据拥有形式的可变借用，若数据处于借用形式，则会克隆数据                                    |
| `into_owned` | 提取拥有的数据，若数据处于借用模式，则会克隆数据 |


## 代码示例

### 字符串
```rust
fn main() {
    let s: &str = "Rustacean";
    let mut c1 = Cow::from(s); // Cow::Borrowed

    let s: String = String::from("Rust");
    let mut c2 = Cow::from(s); // Cow::Owned

    print_cow_string(&c1, &c2);  // Deref 自动解引用
    print_pair((&c1, &c2));

    c1.to_mut().push_str(" Guide"); // to_mut 克隆数据并获得可变借用 Cow::Borrowed -> Cow::Owned
    c2.to_mut().push_str(" Guide"); // Cow::Owned -> Cow::Owned

    print_pair((&c1, &c2));

    let s: &str = "Rustacean";
    let c = Cow::from(s);
    let o: String = c.into_owned(); // 克隆 &str 的数据变成 String，提取使用
    println!("{}", o);
}

fn print_cow_string(s1: &str, s2: &str) {
    println!("&str: {}, String: {}", s1, s2);
}

fn print_pair(pair: (&Cow<str>, &Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: &Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Cow::Borrowed=>{}", v),
        Cow::Owned(ref v) => format!("Cow::Owned=>{}", v),
    }
}
```

### 数组
```rust
fn main() {
    let slice = [1, 2, 3];
    let mut input = Cow::from(&slice[..]);
    print_cow(&input);
    abs_all(&mut input);
    print_cow(&input);

    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    print_cow(&input);
    abs_all(&mut input);
    print_cow(&input);

    let vec = vec![-1, 0, 1];
    let mut input = Cow::from(vec);
    print_cow(&input);
    abs_all(&mut input);
    print_cow(&input);
}

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}

fn print_cow(input: &Cow<[i32]>) {
    match input {
        Cow::Borrowed(_) => println!("Cow::Borrowed."),
        Cow::Owned(_) => println!("Cow::Owned."),
    }
}
```