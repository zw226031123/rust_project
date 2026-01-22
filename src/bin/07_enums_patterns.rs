// ============================================================================
// Rust 学习笔记 07: 枚举与模式匹配
// ============================================================================
//
// 枚举（Enum）允许你定义一个类型，该类型可以是多个变体中的一个。
// 模式匹配（Pattern Matching）允许你根据值的结构来执行不同的代码。
// 结合使用枚举和 match 表达式是 Rust 中最强大的特性之一。
//
// ============================================================================

fn main() {
    // ========== 1. 定义枚举 ==========
    println!("=== 枚举基础 ===");

    // 简单枚举
    let coin = Coin::Penny;
    println!("{:?} 的值: {} 分", coin, coin.value());

    let coin2 = Coin::Quarter(UsState::California);
    println!("{:?} 的值: {} 分", coin2, coin2.value());

    // ========== 2. Option 枚举 ==========
    println!("\n=== Option 枚举 ===");

    // Option<T> 表示一个值可能存在（Some）也可能不存在（None）
    let some_number = Some(42);
    let some_string = Some("hello");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    // 使用 match 处理 Option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);

    // ========== 3. Result 枚举 ==========
    println!("\n=== Result 枚举 ===");

    // Result<T, E> 用于可能成功或失败的操作
    // Ok(T) 表示成功，E 表示错误
    let success: Result<i32, &str> = Ok(200);
    let failure: Result<i32, &str> = Err("Not Found");

    println!("success: {:?}", success);
    println!("failure: {:?}", failure);

    // 使用 unwrap 获取值，或 panic
    println!("success.unwrap() = {}", success.unwrap());
    // println!("failure.unwrap()); // panic!

    // ========== 4. match 表达式 ==========
    println!("\n=== match 表达式 ===");

    let value = 5;
    let description = match value {
        1 => "一",
        2 => "二",
        3 => "三",
        4 => "四",
        5 => "五",
        _ => "其他", // 匹配所有剩余情况
    };
    println!("{} 是 {}", value, description);

    // ========== 5. 模式匹配变体 ==========
    println!("\n=== 高级模式匹配 ===");

    // 匹配枚举携带的数据
    let msg = Message::Write(String::from("hello"));
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("写入: {}", text),
        Message::ChangeColor(r, g, b) => println!("颜色: RGB({}, {}, {})", r, g, b),
    }

    // 绑定模式 - 在匹配时提取值
    let optional = Some(10);
    if let Some(value) = optional {
        println!("获取到值: {}", value);
    }

    // 匹配守卫
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("偶数: {}", x),
        Some(x) => println!("奇数: {}", x),
        None => println!("没有值"),
    }

    // 多重模式
    let x = 5;
    match x {
        1 | 2 | 3 | 4 | 5 => println!("1-5 范围内"),
        _ => println!("其他"),
    }

    // 范围模式
    let x = 'c';
    match x {
        'a'..='m' => println!("前半部分字母"),
        'n'..='z' => println!("后半部分字母"),
        _ => println!("其他字符"),
    }

    // ========== 6. 解构结构体和枚举 ==========
    println!("\n=== 解构模式 ===");

    let person = Person {
        name: String::from("Alice"),
        age: 30,
        address: Address {
            city: String::from("Beijing"),
            zip: "100000",
        },
    };

    // 解构整个结构体
    let Person {
        name,
        age,
        address: Address { city, zip: _ },
    } = &person;
    println!("{} 的 {} 岁，住在 {}", name, age, city);

    // 解构用于 match
    match person {
        Person { name, age: 30, .. } => println!("{} 刚好30岁", name),
        Person { name, age, .. } => println!("{} 是 {} 岁", name, age),
    }

    // ========== 7. if let 和 while let ==========
    println!("\n=== if let 和 while let ===");

    // if let - 简化单分支 match
    let config_max = Some(100u8);
    if let Some(max) = config_max {
        println!("最大配置值: {}", max);
    }

    // if let 带有 else
    let config: Option<i32> = None;
    if let Some(value) = config {
        println!("配置值: {}", value);
    } else {
        println!("没有配置");
    }

    // while let - 只要模式匹配就继续循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("栈元素:");
    while let Some(top) = stack.pop() {
        println!("  {}", top);
    }

    // ========== 8. 闭包中的模式匹配 ==========
    println!("\n=== 函数式风格 ===");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 使用迭代器适配器
    let sum: i32 = numbers.iter().sum();
    println!("总和: {}", sum);

    let even_sum: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // 模式匹配：解构元组
        .sum();
    println!("偶数和: {}", even_sum);
}

// ============================================================================
// 枚举定义
// ============================================================================

// 简单枚举
#[allow(unused)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 携带数据的变体
}

// 为枚举实现方法
impl Coin {
    fn value(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(_) => 25,
        }
    }
}

// 用于 Coin::Quarter 的枚举
#[allow(unused)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    NewYork,
    // ... 其他州
}

// 复杂枚举
#[allow(unused)]
#[derive(Debug)]
enum Message {
    Quit,                    // 无数据的变体
    Move { x: i32, y: i32 }, // 具名字段
    Write(String),           // 单个值
    ChangeColor(u8, u8, u8), // 元组风格
}

// 为 Message 实现方法
#[allow(unused)]
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit 消息"),
            Message::Move { x, y } => println!("Move 到 ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
        }
    }
}

// ========== 结构体定义 ==========

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: Address,
}
#[allow(unused)]
#[derive(Debug)]
struct Address {
    city: String,
    zip: &'static str,
}

// ============================================================================
// 总结
// ============================================================================
//
// 1. 枚举使用 enum 关键字定义，变体使用 VariantName 语法
// 2. 枚举变体可以携带数据（类似联合体/标签联合）
// 3. Option<T> 是标准库定义的枚举，表示值可能存在或不存在
// 4. Result<T, E> 是标准库定义的枚举，表示成功或失败
// 5. match 必须穷举所有情况，_ 匹配其余情况
// 6. if let 简化单分支 match
// 7. while let 循环只要模式匹配就继续
// 8. 模式可以解构复杂数据结构
// 9. 匹配守卫（if condition）添加额外条件
// 10. | 用于多重模式，.. 用于范围模式
//
