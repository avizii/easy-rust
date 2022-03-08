use std::mem::size_of_val;

// https://mcll.top/2020/03/09/rust-closure/
// https://juejin.cn/post/6991378217701474334
// https://blog.linyinfeng.com/posts/how-do-rust-closures-work/
// https://m.yisu.com/zixun/542947.html
// https://rustcc.cn/article?id=cd3afc4e-58f5-41fd-8a91-cd4accb78b92

fn main() {
    fn_test();
}

fn fn_test() {
    println!("start to call fn_once ......");
    let s = String::from("Rustacean");
    let mut f1 = |x: u64| s.len() as u64 * x;
    let mut f2 = |x: u64| s.len() as u64 * x;
    let mut f3 = |x: u64| s.len() as u64 * x;
    println!("call fn_once: {}", call_fn_once_1(1, f1));
    // println!("call fn_once: {}", call_fn_once_2(1, Box::new(f2)));
    println!("call fn_once: {}", call_fn_once_3(1, f3));

    println!("start to call fn_mut ......");
    let mut s = String::from("Rust");
    let mut f = |x: u64| s.len() as u64 * x;
    println!("call fn_mut: {}", call_fn_mut_1(1, &mut f));
    // println!("call fn_mut: {}", call_fn_mut_2(1, &mut f));
    println!("call fn_mut: {}", call_fn_mut_3(1, &mut f));

    println!("start to call fn ......");
    let s = String::from("Closure");
    let f = |x: u64| s.len() as u64 * x;
    println!("call fn: {}", call_fn_1(1, &f));
    // println!("call fn: {}", call_fn_2(1, &f));
    println!("call fn: {}", call_fn_3(1, &f));
}

fn call_fn_once_1(arg: u64, f: impl FnOnce(u64) -> u64) -> u64 {
    f(arg)
}

fn call_fn_once_2(arg: u64, f: Box<dyn FnOnce(u64) -> u64>) -> u64 {
    f(arg)
}

fn call_fn_once_3<F>(arg: u64, f: F) -> u64
    where
        F: FnOnce(u64) -> u64,
{
    f(arg)
}

fn call_fn_mut_1(arg: u64, f: &mut impl FnMut(u64) -> u64) -> u64 {
    f(arg)
}

fn call_fn_mut_2(arg: u64, mut f: Box<&mut dyn FnMut(u64) -> u64>) -> u64 {
    f(arg)
}

fn call_fn_mut_3<F>(arg: u64, f: &mut F) -> u64
    where
        F: FnMut(u64) -> u64,
{
    f(arg)
}

fn call_fn_1(arg: u64, f: &impl Fn(u64) -> u64) -> u64 {
    f(arg)
}

fn call_fn_2(arg: u64, f: Box<&dyn Fn(u64) -> u64>) -> u64 {
    f(arg)
}

fn call_fn_3<F>(arg: u64, f: &F) -> u64
    where
        F: Fn(u64) -> u64,
{
    f(arg)
}