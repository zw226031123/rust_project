// ============================================================================
// Rust 学习笔记 05: 所有权系统
// ============================================================================
//
// 所有权是 Rust 最独特的特性，使 Rust 能够在没有垃圾回收的情况下保证内存安全。
//
// 所有权规则：
// 1. Rust 中每一个值都有一个所有者（owner）
// 2. 同一时间只能有一个所有者
// 3. 当所有者离开作用域时，值会被自动丢弃
//
// ============================================================================

fn main() {
    // ========== 1. 所有权与作用域 ==========
    println!("=== 所有权基础 ===");

    {
        // s 在这里无效，尚未声明
        let s = "hello"; // s 在这里有效
        println!("s 在作用域内: {}", s);
    } // s 在这里失效

    // println!("{}", s); // 错误！s 已经离开作用域

    // ========== 2. String 类型的所有权 ==========
    // &str 是字符串字面量，存储在二进制中，不可变
    // String 存储在堆上，可变

    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2

    // println!("{}", s1); // 错误！s1 不再有效
    println!("s2: {}", s2);

    // 如果想要复制而不是移动，使用 clone()
    let s3 = String::from("world");
    let s4 = s3.clone(); // 显式深拷贝
    println!("s3: {}, s4: {}", s3, s4);

    // ========== 3. 基本类型的拷贝 ==========
    // 基本类型（整数、浮点数、布尔、字符、元组）存储在栈上
    // 它们实现了 Copy trait，会自动拷贝

    let x = 5;
    let y = x; // 自动拷贝
    println!("x = {}, y = {}", x, y); // x 仍然有效

    // 只有 Drop trait 的类型不能是 Copy 的
    // 如果一个类型有析构函数（Drop trait），就不能实现 Copy

    // ========== 4. 函数中的所有权 ==========
    println!("\n=== 函数中的所有权 ===");

    let s = String::from("hello");
    println!("调用函数前: s = {}", s);

    takes_ownership(s); // s 的所有权移动到函数中
    // println!("{}", s); // 错误！s 已经无效

    let x = 5;
    makes_copy(x); // x 是 Copy 的，会拷贝
    println!("调用函数后: x = {}", x); // x 仍然有效

    // ========== 5. 返回值与所有权 ==========
    println!("\n=== 返回值与所有权 ===");

    let s1 = gives_ownership(); // 返回值移动到 s1
    println!("s1: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 移动到函数，函数返回值移动到 s3
    // println!("{}", s2); // 错误！s2 已经无效
    println!("s3: {}", s3);

    // ========== 6. 引用与借用 ==========
    println!("\n=== 引用与借用 ===");

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 创建对 s1 的引用（借用）
    println!("'{}' 的长度是 {}", s1, len); // s1 仍然有效

    // 引用的默认值是不可变的
    // let s = &s1;
    // s.push_str(" world"); // 错误！引用不可变

    // ========== 7. 可变引用 ==========
    println!("\n=== 可变引用 ===");

    let mut s = String::from("hello");
    println!("修改前: {}", s);

    change(&mut s); // 可变引用
    println!("修改后: {}", s);

    // 可变引用的限制：在同一作用域，同一时间只能有一个可变引用
    // 这防止了数据竞争

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // 错误！不能同时有多个可变引用
    println!("r1: {}", r1);

    // 不可变引用可以有多个
    let r2 = &s;
    let r3 = &s;
    println!("r2: {}, r3: {}", r2, r3);

    // 但是不能在有不可变引用时创建可变引用
    // let r4 = &mut s; // 错误！

    // ========== 8. 悬垂引用 ==========
    println!("\n=== 悬垂引用（编译时捕获） ===");

    // 下面的代码无法编译，编译器会捕获悬垂引用
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x; // x 在这里失效
    //     }
    //     println!("{}", r); // 错误！r 是悬垂引用
    // }

    println!("悬垂引用在编译时会被捕获！");

    // ========== 9. 借用规则总结 ==========
    println!("\n=== 借用规则 ===");
    println!("1. 引用的作用域从声明处到最后一次使用处");
    println!("2. 同时只能有一个可变引用");
    println!("3. 同时可以有多个不可变引用");
    println!("4. 不可变引用存在时，不能有可变引用");

    // ========== 10. Slice 类型 ==========
    println!("\n=== Slice 类型 ===");

    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);

    // 字符串字面量是 slice
    let s2 = "hello world";
    let word2 = first_word2(s2);
    println!("第一个单词: {}", word2);

    // 字符串 slice 是 &str
    let s3 = String::from("hello world");
    let hello = &s3[..5]; // &str 类型
    let world = &s3[6..]; // &str 类型
    println!("hello: '{}', world: '{}'", hello, world);
}

// ============================================================================
// 函数定义
// ============================================================================

fn takes_ownership(s: String) {
    println!("函数中: {}", s);
} // s 在这里被丢弃

fn makes_copy(x: i32) {
    println!("函数中: {}", x);
} // x 是 Copy 的，离开作用域后会自动拷贝

fn gives_ownership() -> String {
    let s = String::from("hello");
    s // 所有权返回给调用者
}

fn takes_and_gives_back(s: String) -> String {
    s // 所有权返回给调用者
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 是引用，不会丢弃

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return i;
        }
    }
    s.len()
}

// 使用 &str 代替 &String（更通用）
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s
}

// ============================================================================
// 总结
// ============================================================================
//
// 1. 每个值只有一个所有者
// 2. 所有者离开作用域时，值被丢弃
// 3. 移动（move）转移所有权，克隆（clone）深拷贝
// 4. 基本类型自动实现 Copy trait
// 5. 引用（&）借用值，不获得所有权
// 6. 可变引用（&mut）可以修改值
// 7. 借用规则防止数据竞争和悬垂引用
// 8. Slice (&str) 提供对集合一部分的引用
//
