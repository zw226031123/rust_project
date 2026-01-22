// ============================================================================
// Rust 学习笔记 12: Trait
// ============================================================================
//
// Trait 定义了类型可以实现的行为（方法）。
// 类似于其他语言的接口（interface），但更加强大。
// Trait 是 Rust 实现多态和代码重用的核心机制。
//
// ============================================================================

use std::fmt::{self, Debug, Display};
use std::ops::Add;

fn main() {
    println!("=== Trait 系统 ===\n");

    // ========== 1. 定义和实现 Trait ==========
    println!("1. 定义和实现 Trait:");

    let dog = Dog {
        name: String::from("Buddy"),
        age: 3,
    };
    // println!("   {}", dog.speak());
    println!("   {}", dog.who_am_i());

    // ========== 2. Trait 作为参数 ==========
    println!("\n2. Trait 作为参数:");

    fn notify(item: &impl Summary) {
        println!("通知: {}", item.summarize());
    }

    let article = NewsArticle {
        headline: String::from("Rust 1.0 发布"),
        location: String::from("全球"),
        author: String::from("Rust 团队"),
        content: String::from("Rust 编程语言正式发布..."),
    };

    notify(&article);

    // ========== 3. Trait 约束语法 ==========
    println!("\n3. Trait 约束语法:");

    // 语法1: impl Trait（简洁）
    fn notify1(item: &impl Summary) {
        println!("{}", item.summarize());
    }

    // 语法2: trait bound（更灵活）
    fn notify2<T: Summary>(item: &T) {
        println!("{}", item.summarize());
    }

    // 多个约束
    #[allow(dead_code)]
    fn notify3(item: &(impl Summary + Display)) {
        println!("{}", item);
    }

    #[allow(dead_code)]
    fn notify4<T: Summary + Display>(item: &T) {
        println!("{}", item);
    }

    // where 子句
    #[allow(dead_code)]
    fn notify5<T>(item: &T)
    where
        T: Summary + Display,
    {
        println!("{}", item);
    }

    notify1(&article);
    notify2(&article);
    // notify5(&article);

    // ========== 4. 返回 Trait 类型 ==========
    println!("\n4. 返回 Trait 类型:");

    fn returns_summarizable() -> impl Summary {
        NewsArticle {
            headline: String::from("返回 trait"),
            location: String::from("这里"),
            author: String::from("某人"),
            content: String::from("内容..."),
        }
    }

    let summary = returns_summarizable();
    println!("   {}", summary.summarize());

    // ========== 5. Trait 继承 (Supertrait) ==========
    println!("\n5. Trait 继承:");

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("   {}", person);

    // ========== 6. 条件实现 (Blanket Implementations) ==========
    println!("\n6. 条件实现:");

    // 为所有实现 Display 和 Clone 的 T 实现 Summary
    // 这是 Rust 泛型系统的强大特性

    #[derive(Clone, Debug)]
    struct Book {
        title: String,
        author: String,
    }

    // 泛型 trait 实现
    let book = Book {
        title: String::from("Rust 编程"),
        author: String::from("Mozilla"),
    };

    println!("   书: {} by {}", book.title, book.author);

    // ========== 7. 派生 Trait ==========
    println!("\n7. 派生 Trait:");

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    println!("   p1: {:?}", p1);
    println!("   p2: {:?}", p2);
    println!("   p1 == p2: {}", p1 == p2);

    // ========== 8. 默认方法 ==========
    println!("\n8. 默认方法:");

    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("Hello Rust!"),
        reply: false,
        retweet: false,
    };

    // println!("   {}", tweet.summarize());
    println!("   {}", DefaultSummary::summarize(&tweet));
    println!("   {}", Summary::summarize(&tweet));
    // 使用默认实现
    println!("   默认摘要: {}", tweet.default_summary());

    // ========== 9. Operator Overloading ==========
    println!("\n9. 运算符重载:");

    #[derive(Debug, Copy, Clone)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + rhs.0 * 1000)
        }
    }

    let mm = Millimeters(500);
    let m = Meters(2);
    let result = mm + m;
    println!("   500mm + 2m = {}mm", result.0);

    // ========== 10. 完全限定语法 ==========
    println!("\n10. 完全限定语法:");

    let person = Person {
        name: String::from("Bob"),
        age: 25,
    };

    // 调用特定实现
    println!("   Display: {}", person);
    println!("   自我介绍: {}", person.who_am_i());

    // ========== 11. new 和关联函数 ==========
    println!("\n11. new 作为 trait:");

    let rect = Rectangle::new(10, 20);
    println!("   矩形: {:?}", rect);

    // ========== 12. Drop trait ==========
    println!("\n12. Drop trait:");

    struct Droppable {
        name: String,
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("   释放: {}", self.name);
        }
    }

    {
        let _d1 = Droppable {
            name: "第一个".to_string(),
        };
        let _d2 = Droppable {
            name: "第二个".to_string(),
        };
        println!("   在作用域内");
    }
    println!("   作用域结束");
}

// ============================================================================
// Trait 定义
// ============================================================================

// 基本 trait
trait Summary {
    // 方法签名（必须有实现或默认实现）
    fn summarize(&self) -> String;
}

// 带默认实现的 trait
trait DefaultSummary {
    fn summarize(&self) -> String {
        String::from("(阅读更多...)")
    }

    fn default_summary(&self) -> String {
        self.summarize()
    }
}

// Supertrait：OutlinePrint 必须先实现 Display
#[allow(dead_code)]
trait OutlinePrint: Display {
    fn outline_format(&self) -> String {
        format!("* {} *", self.to_string())
    }
}

//=============================================================================
// 为类型实现 Trait
//=============================================================================

#[derive(Debug)]
struct Dog {
    name: String,
    age: u32,
}

// 为 Dog 实现 Summary
impl Summary for Dog {
    fn summarize(&self) -> String {
        format!("{} 是 {} 岁的狗", self.name, self.age)
    }
}

// 实现多个 trait
impl Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

trait Identify {
    fn who_am_i(&self) -> String;
}

impl Identify for Dog {
    fn who_am_i(&self) -> String {
        format!("我是 {}, 品种是狗", self.name)
    }
}

//=============================================================================
// 结构体实现 trait
//=============================================================================

#[allow(dead_code)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[allow(dead_code)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

impl DefaultSummary for Tweet {}

//=============================================================================
// 泛型实现 trait
//=============================================================================

// 泛型 trait
#[allow(dead_code)]
trait Contains<A, B> {
    fn contains(&self, a: &A, b: &B) -> bool;
}

// 结构体使用关联类型
#[allow(dead_code)]
trait Container {
    type Item;

    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn add(&mut self, item: Self::Item);
}

//=============================================================================
// Supertrait 示例
//=============================================================================

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}岁)", self.name, self.age)
    }
}

impl OutlinePrint for Person {}

impl Identify for Person {
    fn who_am_i(&self) -> String {
        format!("我是 {}, {}岁", self.name, self.age)
    }
}

//=============================================================================
// Drop trait 示例
//=============================================================================

// Drop trait 在值离开作用域时自动调用
// 用于释放资源（文件句柄、网络连接等）
#[allow(dead_code)]
struct File {
    name: String,
}

impl Drop for File {
    fn drop(&mut self) {
        println!("   关闭文件: {}", self.name);
    }
}

//=============================================================================
// new pattern
//=============================================================================

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

trait New {
    fn new(width: u32, height: u32) -> Self;
}

impl New for Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

//=============================================================================
// 运算符重载示例
//=============================================================================

// 为自定义类型实现 Add
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

//=============================================================================
// 总结
// ============================================================================
//
// 1. trait 定义共享行为（类似接口）
// 2. impl traitName for Type 实现 trait
// 3. trait 可以有默认方法实现
// 4. trait 作为参数: fn foo(item: &impl Trait)
// 5. trait 约束: fn foo<T: Trait>(item: &T)
// 6. where 子句: fn foo<T>(item: &T) where T: Trait
// 7. 返回 trait: fn foo() -> impl Trait
// 8. Supertrait: trait Sub: Super
// 9. 条件实现: impl<T> Trait for T where T: OtherTrait
// 10. 完全限定语法: <Type as Trait>::method()
// 11. Derive: #[derive(Trait)] 自动实现常用 trait
// 12. Drop trait 用于资源清理
//
//=============================================================================
