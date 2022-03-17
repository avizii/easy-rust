use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {}


// generic
//  1.使用泛型参数延迟数据结构的绑定
//  2.使用泛型参数和 PhantomData，声明数据结构中不直接使用，但在实现过程中需要用到的类型
//  3.使用泛型参数让同一个数据结构对同一个 trait 可以拥有不同的实现

// 在定义数据结构时，对于额外的、暂时不需要的泛型参数，用 PhantomData 来拥有他们，来规避编译器的报错
// 用泛型数据结构来统一相同的逻辑，用泛型参数的具体类型来处理变化的逻辑(对于所定义的泛型参数，在数据结构定义的时候其实并不需要，只是在数据结构的实现过程中，才需要用到它们的约束)
// 在 trait 中，不知使用 impl trait 做返回值，当返回值需要携带泛型参数时，可使用特征对象 trait object

#[derive(Debug, Default, PartialEq, Eq)]
struct Identifier<T> {
    inner: u64,
    _tag: PhantomData<T>,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct User {
    id: Identifier<Self>,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Product {
    id: Identifier<Self>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_should_not_be_the_same() {
        let user = User::default();
        let product = Product::default();

        // assert_ne!(user.id, product.id);

        assert_eq!(user.id.inner, product.id.inner);
    }
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

trait Free {
    fn feature1(&self);

    fn feature2(&self);
}

trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name);
    }

    fn feature2(&self) {
        println!("feature 2 for {}", self.name);
    }
}

struct FreePlan;

struct PersonalPlan(f32);

impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!("dear {}(as our valuable customer {}), enjoy this advanced feature!", self.name, self.id);
    }
}

impl<T> Customer<T> {
    fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name)
    }
}

fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);
    customer.into()
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_customer() {
        let customer = Customer::<FreePlan>::new("avizii".into());
        customer.feature1();
        customer.feature2();

        let customer = subscribe(customer, 8.88);
        customer.feature1();
        customer.feature2();
        customer.advance_feature();
    }
}

#[derive(Debug, Default)]
struct Equation<IterMethod> {
    current: u32,
    _method: PhantomData<IterMethod>,
}

#[derive(Debug, Default)]
struct Linear;

#[derive(Debug, Default)]
struct Quadratic;

impl Iterator for Equation<Linear> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u32::MAX {
            return None;
        }
        Some(self.current)
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u16::MAX as u32 {
            return None;
        }
        Some(self.current * self.current)
    }
}

#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    fn test_linear() {
        let mut equation = Equation::<Linear>::default();
        assert_eq!(Some(1), equation.next());
        assert_eq!(Some(2), equation.next());
        assert_eq!(Some(3), equation.next());
    }

    #[test]
    fn test_quadratic() {
        let mut equation = Equation::<Quadratic>::default();
        assert_eq!(Some(1), equation.next());
        assert_eq!(Some(4), equation.next());
        assert_eq!(Some(9), equation.next());
    }
}

trait ImplTrait {
    fn impl_in_args(s: impl Into<String>) -> String {
        s.into()
    }

    // error 不支持在 trait 里使用 impl trait 做返回值
    // fn impl_as_return(s: String) -> impl Into<String> {
    //     s
    // }
}

fn impl_as_return(s: String) -> impl Into<String> {
    s
}

fn consume_iterator<F, Iter, T>(mut f: F)
    where
        F: FnMut(i32) -> Iter,
        Iter: Iterator<Item=T>,
        T: Debug,
{
    for item in f(10) {
        println!("{:?}", item);
    }
}

#[cfg(test)]
mod test3 {
    use super::*;

    #[test]
    fn test_consume_iterator() {
        let f = |i| (0..i).into_iter();
        consume_iterator(f);
    }
}

/// trait object
// 当在某个上下文中需要满足某个 trait 的类型，且这样的类型可能有很多，当前上下文无法确定会得到哪一个类型时，
// 我们可以用 trait object 来统一处理行为
// trait object 把决策延迟到运行时，带来的后果是执行 效率的打折

// 在函数参数中使用 trait object
type BoxedError = Box<dyn Error + Send + Sync>;

trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedError>;
}

struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

impl<'a, 'b> Shell<'a, 'b> {
    fn new(cmd: &'a str, args: &'b [&'a str]) -> Self {
        Self {
            cmd,
            args
        }
    }
}

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedError> {
        let output = Command::new(self.cmd).args(self.args).output()?;
        Ok(output.status.code())
    }
}

fn execute_generics(cmd: &impl Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

fn execute_boxed_trait_object(cmd: Box<dyn Executor>) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

#[cfg(test)]
mod test4 {
    use super::*;

    #[test]
    fn shell_should_work() {
        let cmd = Shell::new("ls", &[]);
        let res = cmd.run().unwrap();
        assert_eq!(res, Some(0));
    }

    #[test]
    fn execute_should_work() {
        let cmd = Shell::new("ls", &[]);

        let res = execute_generics(&cmd).unwrap();
        assert_eq!(res, Some(0));

        let res = execute_trait_object(&cmd).unwrap();
        assert_eq!(res, Some(0));

        let boxed = Box::new(cmd);
        let res = execute_boxed_trait_object(boxed).unwrap();
        assert_eq!(res, Some(0));

    }
}