// ============================================================================
// Rust 学习笔记 13: 生命周期
// ============================================================================
//
// 生命周期是 Rust 最独特也最具挑战性的特性之一。
// 它们确保引用始终有效，防止悬垂引用。
//
// 核心概念：
// - 生命周期标注告诉编译器引用的有效范围
// - Rust 编译器使用借用检查器验证所有借用都是有效的
// - 大多数情况下，编译器可以推断生命周期，不需要手动标注
//
// ============================================================================

use std::fmt::Display;

fn main() {
    println!("=== 生命周期详解 ===\n");

    // ========== 1. 什么是生命周期 ==========
    println!("1. 什么是生命周期:");

    {
        let x = 5; // x 的生命周期开始
        println!("x = {}", x); // x 在这里有效
    } // x 的生命周期结束

    // println!("{}", x); // 错误！x 已经离开作用域

    // ========== 2. 悬垂引用问题 ==========
    println!("\n2. 悬垂引用:");

    // 下面的代码无法编译，展示了悬垂引用的问题：
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;  // x 的生命周期比 r 短
    //     }
    //     println!("r = {}", r); // 错误！x 已经离开作用域
    // }

    println!("   悬垂引用在编译时会被检测并拒绝！");

    // ========== 3. 借用检查器 ==========
    println!("\n3. 借用检查器:");

    // Rust 使用借用检查器确保引用始终有效
    let mut s1 = String::from("hello");

    let r1 = &s1; // 不可变引用
    let r2 = &s1; // 另一个不可变引用
    println!("r1 = {}, r2 = {}", r1, r2);

    let r3 = &mut s1; // 可变引用
    // r3 可以修改 s1
    println!("r3 = {}", r3);

    // ========== 4. 生命周期标注语法 ==========
    println!("\n4. 生命周期标注语法:");

    // 生命周期参数使用 ' 开头（如 'a, 'b）
    // 它们表示引用的"有效时间"

    let string1 = String::from("long string");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("   最长的字符串是: '{}'", result);

    // ========== 5. 生命周期标注在结构体中 ==========
    println!("\n5. 生命周期标注在结构体中:");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("   引文: {}", i.part);

    // ========== 6. 方法中的生命周期 ==========
    println!("\n6. 方法中的生命周期:");

    let chapter = String::from("第一章");
    let excerpt = ImportantExcerpt {
        part: chapter.as_str(),
    };

    println!("   引文: {}", excerpt.announce_and_return_part("读者"));

    // ========== 7. 静态生命周期 ==========
    println!("\n7. 静态生命周期:");

    // 'static 意味着引用在程序整个运行期间都有效
    let s: &'static str = "我有一个静态生命周期";
    println!("   {}", s);

    // 字符串字面量有 'static 生命周期
    let message = "Hello, Rust!";
    println!("   字符串字面量: {}", message);

    // ========== 8. 泛型生命周期参数 ==========
    println!("\n8. 泛型生命周期参数:");

    let string1 = String::from("a");
    let string2 = String::from("xyz");

    println!(
        "   longest_with_announcement: {}",
        longest_with_announcement(&string1, &string2, "正在查找最长的字符串")
    );

    // ========== 9. 生命周期省略规则 ==========
    println!("\n9. 生命周期省略规则:");

    // Rust 会自动推断以下情况：
    // 1. 每个引用参数都有自己的生命周期
    // 2. 如果只有一个输入生命周期，它被赋给所有输出生命周期
    // 3. 如果有多个输入生命周期且其中一个是 &self 或 &mut self，
    //    那么 self 的生命周期赋给所有输出生命周期

    let s = String::from("hello");
    let result = first_word(&s); // Rust 自动推断
    println!("   第一个单词: {}", result);

    // ========== 10. 生命周期与结构体 ==========
    println!("\n10. 生命周期与结构体:");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Ref<'a, T> {
        part: &'a T,
    }

    let book_title = String::from("Rust Programming");
    let ref_struct = Ref { part: &book_title };
    println!("   Ref 结构体: {:?}", ref_struct);

    // ========== 11. 综合示例 ==========
    println!("\n11. 综合示例:");

    let author = String::from("James");
    let book = Book {
        author: &author,
        title: String::from("Rust in Action"),
        year: 2024,
    };

    println!("   书名: {}", book.title);
    println!("   作者: {}", book.author);
    println!("   年份: {}", book.year);

    let reference = book.get_reference();
    println!("   引用: {}", reference);
}

// ============================================================================
// 函数中的生命周期
// ============================================================================

// longest 函数：返回两个字符串切片中较长的一个
// 'a 表示返回引用的生命周期与两个输入引用的生命周期相同
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// 生命周期标注在泛型中的使用
fn longest_with_announcement<'a, T>(s1: &'a str, s2: &'a str, announcement: T) -> &'a str
where
    T: Display,
{
    println!("   公告: {}", announcement);
    if s1.len() > s2.len() { s1 } else { s2 }
}

// 省略规则示例（不需要显式标注）
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s
}

// ============================================================================
// 结构体中的生命周期
// ============================================================================

// ImportantExcerpt 持有字符串切片的引用
// 它确保只要 ImportantExcerpt 存在，引用的字符串就存在
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 为结构体实现方法
impl<'a> ImportantExcerpt<'a> {
    // 规则3：如果有 &self，所有输出生命周期与 &self 相同
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("   公告: {}", announcement);
        self.part
    }
}

// 多个生命周期参数
#[allow(dead_code)]
struct Ref<'a, 'b, T> {
    x: &'a T,
    y: &'b T,
}

// ============================================================================
// 综合示例：Book 结构体
// ============================================================================

#[derive(Debug)]
struct Book<'a> {
    author: &'a str,
    title: String,
    year: u32,
}

impl<'a> Book<'a> {
    #[allow(dead_code)]
    fn get_reference(&self) -> &str {
        self.author
    }

    // 返回值的生命周期与 self 相同
    #[allow(dead_code)]
    fn get_title(&self) -> &str {
        &self.title
    }
}

// ============================================================================
// 生命周期变体
// ============================================================================

// 1. 生命周期变体
#[allow(dead_code)]
struct Owner {
    name: String,
}

// 引用者
#[allow(dead_code)]
struct Borrower<'a> {
    owner: &'a Owner,
}

// 2. 生命周期在闭包中
#[allow(dead_code)]
fn exec_with_callback<F>(callback: F)
where
    F: FnOnce(),
{
    callback();
}

// ============================================================================
// 生命周期常见错误和解决方案
// ============================================================================

/*
// 错误示例1：函数返回的引用可能无效
fn bad_function<'a>() -> &'a str {
    let s = String::from("hello");
    &s  // 错误！s 的生命周期太短
}

// 解决方案：返回拥有的值而非引用
fn good_function() -> String {
    let s = String::from("hello");
    s
}

// 错误示例2：结构体持有无效引用
struct BadRef<'a> {
    reference: &'a str,
}

fn create_bad_ref() -> BadRef<'static> {
    let local = String::from("local");
    BadRef { reference: &local } // 错误！
}

// 解决方案：确保引用与结构体同寿
fn create_good_ref(s: &str) -> Ref {
    Ref { reference: s }
}
*/

// ============================================================================
// 何时需要显式标注生命周期
// ============================================================================

/*
需要显式标注的情况：

1. 函数返回引用，且返回值依赖参数之一
2. 结构体包含引用字段
3. 方法返回 self 的引用
4. 泛型函数中需要区分不同生命周期

不需要标注的情况：

1. 省略规则适用的情况
2. 返回值不依赖输入引用（如返回拥有的值）
*/

// ============================================================================
// 总结
// ============================================================================
//
// 1. 生命周期表示引用的有效范围
// 2. 生命周期标注：'a, 'b, 'c 等
// 3. 借用检查器验证所有借用都有效
// 4. 'static 表示整个程序运行期间有效
// 5. 大多数情况下编译器可以推断生命周期
// 6. 函数返回引用时通常需要标注
// 7. 结构体包含引用时需要生命周期参数
// 8. 方法实现通常不需要额外标注（规则3）
// 9. 生命周期确保内存安全，无需 GC
//
