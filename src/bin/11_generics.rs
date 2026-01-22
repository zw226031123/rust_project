// ============================================================================
// Rust 学习笔记 11: 泛型
// ============================================================================
//
// 泛型允许我们编写适用于多种类型的代码，避免代码重复。
// Rust 在编译时会进行单态化（monomorphization），将泛型代码转换为具体类型的代码。
// 这意味着泛型没有运行时开销！
//
// ============================================================================

use std::fmt::Display;

fn main() {
    println!("=== 泛型编程 ===\n");

    // ========== 1. 泛型函数 ==========
    println!("1. 泛型函数:");

    let int_max = max(5, 10);
    let float_max = max(3.14, 2.71);
    let char_max = max('a', 'z');

    println!("   整数最大值: {}", int_max);
    println!("   浮点数最大值: {}", float_max);
    println!("   字符最大值: {}", char_max);

    // ========== 2. 泛型结构体 ==========
    println!("\n2. 泛型结构体:");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let mixed = Point { x: 5, y: 4.0 }; // 不同类型

    println!("   integer: ({}, {})", integer.x, integer.y);
    println!("   float: ({}, {})", float.x, float.y);
    // println!("   mixed: ({}, {})", mixed.x, mixed.y);

    // ========== 3. 泛型枚举 ==========
    println!("\n3. 泛型枚举 (Option):");

    let some_number = Option::<i32>::Some(42);
    let some_string = Option::<String>::Some(String::from("hello"));
    let none_number: Option<i32> = Option::None;

    println!("   some_number: {:?}", some_number);
    println!("   some_string: {:?}", some_string);
    println!("   none_number: {:?}", none_number);

    println!("\n4. 泛型枚举 (Result):");

    let success: Result<i32, &str> = Ok(200);
    let failure: Result<i32, &str> = Err("Not Found");

    println!("   success: {:?}", success);
    println!("   failure: {:?}", failure);

    // ========== 4. 方法中的泛型 ==========
    println!("\n5. 方法中的泛型:");

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 3.0, y: 7.0 };

    println!("   p1.x = {}", p1.x());
    println!("   p2.y = {}", p2.y());

    // 混合类型的方法
    // let p3 = mixed.x_to_string();
    // println!("   mixed.x_to_string() = {}", p3);

    // ========== 5. trait 约束 (where 子句) ==========
    println!("\n6. Trait 约束:");

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let cloned = duplicate(&person);
    println!("   原始: {:?}", person);
    println!("   克隆: {:?}", cloned);

    // ========== 6. 多个泛型参数 ==========
    println!("\n7. 多个泛型参数:");

    let result = combine(10, 20);
    println!("   combine(10, 20) = {}", result);

    // ========== 7. 泛型与 trait ==========
    println!("\n8. 泛型与 Trait 结合:");

    let nums = vec![1, 2, 3, 4, 5];
    let sum: i32 = sum_values(&nums);
    println!("   数组和: {}", sum);

    let floats = vec![1.1, 2.2, 3.3];
    let sum_f: f64 = sum_values(&floats);
    println!("   浮点数和: {}", sum_f);

    // ========== 8. 默认泛型参数 ==========
    println!("\n9. 默认泛型参数:");

    let pair = Pair::new(5, 10);
    println!("   Pair: {:?}", pair);

    // ========== 9. 关联类型 ==========
    println!("\n10. 关联类型:");

    let counter = Counter::new();
    println!("   Counter 迭代: {:?}", counter.collect::<Vec<_>>());

    // ========== 10. 泛型代码的性能 ==========
    println!("\n11. 泛型性能:");

    // 泛型在编译时进行单态化，每个具体类型都会生成一份代码
    // 这意味着：
    // - 编译后的代码没有运行时开销
    // - 但是会增加编译时间和二进制大小

    let a = 42;
    let b = 100;
    println!("   max(i32): {}", max(a, b));

    let c = 3.14;
    let d = 2.71;
    println!("   max(f64): {}", max(c, d));

    // ========== 11. const 泛型 ==========
    println!("\n12. Const 泛型:");

    // 允许在泛型中使用编译时常量
    let array = [1, 2, 3, 4, 5];
    println!("   数组长度: {}", array.len());
    print_array(&array);

    let arr2 = [1; 10];
    print_array(&arr2);
}

// ============================================================================
// 1. 泛型函数
// ============================================================================

// T 必须实现 PartialOrd trait 才能比较大小
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 多重约束
#[allow(dead_code)]
fn print_and_return<T: Display + Copy>(value: T) {
    println!("值为: {}", value);
}

// where 子句语法（更清晰）
fn sum_values<T>(values: &[T]) -> T
where
    T: for<'a> std::iter::Sum<&'a T> + Copy,
{
    values.iter().sum()
}

// ============================================================================
// 2. 泛型结构体
// ============================================================================

struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// 泛型方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// 为特定类型实现方法
#[allow(dead_code)]
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 跨类型方法
#[allow(dead_code)]
impl<T, U> Point2<T, U> {
    fn x_to_string(&self) -> String
    where
        T: ToString,
    {
        self.x.to_string()
    }
}

// ============================================================================
// 3. 泛型枚举（标准库示例）
// ============================================================================

// enum Option<T> {
//     Some(T),
//     None,
// }
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// ============================================================================
// 4. Trait 约束
// ============================================================================

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

// T 必须实现 Clone trait 才能克隆
fn duplicate<T: Clone>(value: &T) -> T {
    value.clone()
}

// 多个约束
#[allow(dead_code)]
fn compare_and_print<T: PartialOrd + Display>(a: &T, b: &T) {
    if a < b {
        println!("{} < {}", a, b);
    } else if a > b {
        println!("{} > {}", a, b);
    } else {
        println!("{} == {}", a, b);
    }
}

// ============================================================================
// 5. 多个泛型参数
// ============================================================================

fn combine<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b

    // 注意：需要 Add<Output = T> 约束
    // 因为 T::Output 可能不同
}

// ============================================================================
// 6. 默认泛型参数
// ============================================================================

#[allow(dead_code)]
#[derive(Debug)]
struct Pair<T = i32> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

// ============================================================================
// 7. 关联类型
// ============================================================================

#[allow(dead_code)]
trait Container {
    type Item;

    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn add(&mut self, item: Self::Item);
}

struct Counter {
    values: Vec<i32>,
}

impl Counter {
    fn new() -> Self {
        Self {
            values: (1..=5).collect(),
        }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.pop()
    }
}

// ============================================================================
// 8. Const 泛型
// ============================================================================

fn print_array<T: Display, const N: usize>(arr: &[T; N]) {
    print!("   [");
    for (i, item) in arr.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", item);
    }
    println!("]");
}

// ============================================================================
// 总结
// ============================================================================
//
// 1. 泛型使用 <T> 或 <T, U> 语法声明类型参数
// 2. 泛型约束使用冒号（:）或 where 子句
// 3. 常见 trait 约束：Display, Debug, Clone, Copy, PartialEq, PartialOrd
// 4. 方法可以使用 impl<T> 为泛型结构体实现
// 5. 特定类型可以用 impl Point<f64> 实现特殊方法
// 6. 多个泛型参数用逗号分隔
// 7. 默认泛型参数使用 T = DefaultType 语法
// 8. 关联类型使用 type Item; 在 trait 中定义
// 9. Const 泛型允许使用编译时常量
// 10. 泛型在编译时单态化，无运行时开销
//
