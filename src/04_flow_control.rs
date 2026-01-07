// ============================================================================
// Rust 学习笔记 04: 流程控制
// ============================================================================
//
// Rust 提供了多种流程控制方式：
// - if/else 条件表达式
// - loop 无限循环
// - while 条件循环
// - for 迭代循环
// - match 模式匹配（强大且安全）
//
// ============================================================================

fn main() {
    // ========== 1. if/else 条件表达式 ==========
    println!("=== if/else 条件表达式 ===");

    let number = 7;

    if number < 5 {
        println!("小于 5");
    } else if number == 5 {
        println!("等于 5");
    } else {
        println!("大于 5");
    }

    // if 是表达式，可以返回值
    let condition = true;
    let value = if condition {
        100
    } else {
        200
    };
    println!("if 表达式结果: {}", value);

    // 注意：分支返回值类型必须一致
    // let mixed = if condition { 1 } else { "hello" }; // 错误！

    // ========== 2. match 模式匹配 ==========
    println!("\n=== match 模式匹配 ===");

    let coin = Coin::Penny;
    println!("{:?} 的值是 {} 分", coin, value_in_cents(coin));

    let coin2 = Coin::Quarter(UsState::Alaska);
    println!("{:?} 的值是 {} 分", coin2, value_in_cents(coin2));

    // match 绑定值
    let dice_roll = 7;
    match dice_roll {
        1 => println!("一等奖！"),
        2 => println!("二等奖！"),
        3 => println!("三等奖！"),
        other => println!("未中奖，再试一次！"), // 捕获其余情况
    }

    // match 匹配元组
    let point = (3, 5);
    match point {
        (0, 0) => println!("原点"),
        (x, 0) => println!("在 x 轴上，x = {}", x),
        (0, y) => println!("在 y 轴上，y = {}", y),
        (x, y) => println!("在 ({} {})", x, y),
    }

    // match 匹配引用
    let reference: &i32 = &10;
    match reference {
        &val => println!("解引用后的值: {}", val),
    }

    // 使用 ref 关键字获取引用
    let ref reference2 = 20;
    match reference2 {
        &val => println!("直接匹配: {}", val),
    }
    match reference2 {
        ref r => println!("通过 ref 获取引用: {}", r),
    }

    // ========== 3. if let 简洁模式匹配 ==========
    println!("\n=== if let 语法 ===");

    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("是 3！"),
        _ => (),
    }

    // 使用 if let 简化
    if let Some(3) = some_u8_value {
        println!("是 3！");
    }

    // if let 带有 else
    let coin3 = Coin::Dime;
    let count = if let Coin::Quarter(state) = coin3 {
        println!("来自 {:?} 的 25 分！", state);
        25
    } else {
        0
    };
    println!("硬币价值: {} 分", count);

    // ========== 4. while let 条件循环 ==========
    println!("\n=== while let 循环 ===");

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }

    // ========== 5. for 循环 ==========
    println!("\n=== for 循环 ===");

    let arr = [10, 20, 30, 40, 50];

    // 基本 for 循环
    for element in arr {
        println!("元素: {}", element);
    }

    // 使用 enumerate 获取索引
    for (index, value) in arr.iter().enumerate() {
        println!("arr[{}] = {}", index, value);
    }

    // 范围迭代
    println!("0 到 4:");
    for i in 0..5 {
        println!("  {}", i);
    }

    // rev() 反转范围
    println!("4 到 0:");
    for i in (0..5).rev() {
        println!("  {}", i);
    }

    // ========== 6. loop 无限循环 ==========
    println!("\n=== loop 循环 ===");

    // loop 是无限循环，需要使用 break 退出
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // break 可以返回值
        }
    };
    println!("loop 结果: {}", result);

    // 多重循环中使用标签
    let mut matrix = [['x'; 3]; 3];
    let mut found = false;
    'outer: for i in 0..3 {
        for j in 0..3 {
            if matrix[i][j] == 'x' {
                println!("在 ({}, {}) 找到 'x'", i, j);
                found = true;
                break 'outer; // 跳出外层循环
            }
        }
    }
    println!("是否找到: {}", found);

    // ========== 7. while 条件循环 ==========
    println!("\n=== while 循环 ===");

    let mut number = 3;
    while number != 0 {
        println!("倒计时: {}", number);
        number -= 1;
    }
    println!("发射！");

    // ========== 8. 综合示例：fizzbuzz ==========
    println!("\n=== FizzBuzz 示例 ===");
    for i in 1..=20 {
        match (i % 3 == 0, i % 5 == 0) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("{}", i),
        }
    }
}

// ============================================================================
// match 模式匹配中使用的枚举
// ============================================================================

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... 其他州
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自 {:?} 的季度硬币！", state);
            25
        }
    }
}

// ============================================================================
// 总结
// ============================================================================
//
// 1. if/else 是表达式，可以返回值
// 2. match 是强大的模式匹配，必须穷举所有情况
// 3. if let 简化只有一种情况的 match
// 4. while let 类似于 while，但基于模式匹配
// 5. for 循环用于迭代集合
// 6. loop 是无限循环，用 break 退出并返回值
// 7. 循环标签可以指定跳出哪个循环
//
