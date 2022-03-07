use std::borrow::Cow;
use reqwest::Url;

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
    let o: String = c.into_owned();
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

    #[test]
    fn test_cow() {
        cow_trait();
    }
}