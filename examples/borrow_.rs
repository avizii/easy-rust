// http://doc.rust-lang.org/1.58.1/std/borrow/index.html
// http://doc.rust-lang.org/1.58.1/std/clone/index.html
// https://blog.frognew.com/2020/07/rust-cow-smart-pointer.html
// https://blog.frognew.com/2020/07/rust-borrow-toowned-traits.html
// https://dhghomon.github.io/easy_rust/Chapter_42.html

use std::borrow::{Borrow, BorrowMut, Cow};

fn main() {
    borrow_trait();

    borrow_mut_trait();
}


// Borrow trait
fn borrow_trait() {
    // str
    let s1: &str = "easy rust for Rustacean";
    let _s2: &str = s1.borrow(); // impl Borrow<T> for T

    // string
    let s1: String = "Rustacean".to_string();
    let _s2: &str = s1.borrow(); // impl Borrow<str> for String
    let _s3: &String = s1.borrow(); // impl Borrow<T> for T

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

    // custom
    struct CusBor {
        name: String,
    }

    impl Borrow<str> for CusBor {
        fn borrow(&self) -> &str {
            self.name.as_str()
        }
    }

    impl Borrow<String> for CusBor {
        fn borrow(&self) -> &String {
            &self.name
        }
    }

    let c1 = CusBor {
        name: "Rust".to_string()
    };
    let _c2: &CusBor = c1.borrow(); // impl Borrow<T> for T
    let _c3: &str = c1.borrow(); // impl Borrow<str> for CusBor
    let _c4: &String = c1.borrow(); // impl Borrow<String> for CusBor
    let _c5: &CusBor = &c1.borrow();  // impl<'_, T> Borrow<T> for &'_ T

    // impl Borrow<T> for U
    // let u = U;
    // let t: &T = u.borrow();
}

fn borrow_mut_trait() {
    // string
    let mut s1: String = "Rustacean".to_string();
    let _s2: &mut str = s1.borrow_mut(); // impl BorrowMut<str> for String
    let _s3: &mut String = s1.borrow_mut(); // impl<T> BorrowMut<T> for T

    // array
    let mut a1: [i32; 4] = [1, 2, 3, 4];
    let a2: &mut [i32; 4] = a1.borrow_mut();  // impl<T> BorrowMut<T> for T
    assert_eq!(a2, &a1);
    let _a3: &mut [i32] = a1.borrow_mut(); // impl<T, const N: usize> BorrowMut<[T]> for [T; N]

    // vec
    let mut v1: Vec<i32> = vec![1, 2, 3, 4];
    let v2: &mut Vec<i32> = v1.borrow_mut(); // impl<T> BorrowMut<T> for T
    assert_eq!(v2, &v1);
    let _v3: &mut [i32] = v1.borrow_mut(); // impl<T> BorrowMut<[T]> for Vec<T, Global>

    // impl BorrowMut<T> for U
    // let mut u = U;
    // let mut t: &T = u.borrow_mut();
}

fn to_owned_trait() {

}

fn cow_trait() {

}


fn modulo_3(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => "remainder is 0.".into(),
        1 => "remainder is 1.".into(),
        remainder => format!("remainder is {}.", remainder).into(),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulo_3() {
        for num in 1..=6 {
            match modulo_3(num) {
                Cow::Borrowed(msg) => println!("Cow::Borrowed for num: {}, msg: {}", num, msg),
                Cow::Owned(msg) => println!("Cow::Owned for num: {}, msg: {}", num, msg),
            }
        }
    }

    #[test]
    fn test_abs_all() {
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
}