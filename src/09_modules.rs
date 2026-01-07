// ============================================================================
// Rust 学习笔记 09: 包和模块系统
// ============================================================================
//
// Rust 有一个独特而灵活的组织系统：
// - Package (包): Cargo 的构建单位，可以包含多个 crate
// - Crate (crate): 一个库或可执行文件的模块树
// - Module (模块): 用于组织代码、控制可见性的命名空间
// - Path (路径): 用于命名item的方式（结构体、函数、模块等）
//
// ============================================================================

// ========== 使用 use 关键字 ==========
// use 关键字将路径导入作用域，避免重复书写完整路径

use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
use std::io::{self, Write};

// ========== 模块定义 ==========
// 使用 mod 关键字定义模块
// 默认情况下，模块是私有的，需要使用 pub 使其公开

// 内联模块定义
mod front_of_house {
    // pub 关键字使模块/函数/结构体等对外可见
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }

        fn seat_at_table() {
            println!("安排座位");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("接收订单");
        }

        fn serve_order() {
            println!("上菜");
        }

        fn take_payment() {
            println!("收款");
        }
    }

    mod back_of_house {
        // 私有结构体
        struct Chef {
            name: String,
            ..Default::default()
        }

        // 公有关联函数
        pub fn cook() {
            println!("烹饪食物");
        }

        // 私有函数
        fn clean_kitchen() {
            println!("清洁厨房");
        }
    }
}

// 使用 super 访问父模块
mod customer {
    pub fn order_food() {
        // 从 customer 模块访问兄弟模块 hosting
        super::front_of_house::hosting::add_to_waitlist();
        println!("点餐");
    }
}

// ========== 结构体可见性 ==========
mod restaurant {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,        // 公开字段
        seasonal_fruit: String,   // 私有字段
    }

    impl Breakfast {
        // 关联函数可以访问所有字段
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// ========== 类型别名 ==========
mod types {
    // 私有模块
    mod internal {
        pub struct MyType {
            pub value: i32,
        }
    }

    // 重新导出：使 internal::MyType 在 types 模块外可用
    pub use internal::MyType;
}

fn main() {
    println!("=== 包和模块系统 ===\n");

    // ========== 1. 调用公开函数 ==========
    println!("1. 调用模块中的函数：");
    // 使用完整路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 使用 use 导入后简写
    hosting::add_to_waitlist();
    serving::take_order();

    // ========== 2. 使用结构体 ==========
    println!("\n2. 使用结构体：");
    let mut meal = restaurant::Breakfast::summer("Rye");
    println!("{:?}", meal);

    // toast 是公开的，可以修改
    meal.toast = String::from("Wheat");
    println!("修改后: {:?}", meal);

    // ========== 3. 使用枚举变体 ==========
    println!("\n3. 使用枚举：");
    let order = restaurant::Appetizer::Soup;
    println!("{:?}", order);

    // ========== 4. 使用类型别名 ==========
    println!("\n4. 类型别名：");
    let my_type = types::MyType { value: 42 };
    println!("{:?}", my_type);

    // ========== 5. 模块嵌套 ==========
    println!("\n5. 嵌套模块：");
    // front_of_house::hosting::add_to_waitlist()
    // 使用 use 后可以直接调用 hosting::add_to_waitlist()

    // ========== 6. re-exports (重新导出) ==========
    println!("\n6. 重新导出：");
    // types 模块重新导出了 internal::MyType
    // 这样外部代码可以使用 types::MyType 而不需要知道 internal 模块

    // ========== 7. 可见性规则总结 ==========
    println!("\n7. 可见性规则：");
    println!("- 默认所有 item 是私有的");
    println!("- pub 使 item 对外可见");
    println!("- pub(super) 只对父模块可见");
    println!("- pub(crate) 对整个 crate 可见");
    println!("- 结构体使用 pub 但字段默认私有");

    // ========== 8. 实际示例 ==========
    println!("\n8. 实际使用示例：");
    customer::order_food();

    // 错误处理示例
    let result: Result<(), io::Error> = write_to_file();
    match result {
        Ok(()) => println!("写入成功"),
        Err(e) => println!("写入失败: {}", e),
    }

    // HashMap 和 HashSet 使用
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Team A"), 100);
    scores.insert(String::from("Team B"), 85);
    println!("{:?}", scores);

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2); // 重复
    println!("{:?}", set);
}

// ========== 模块外部的文件 ==========
// 在实际项目中，模块通常放在单独的文件中
// 例如：src/front_of_house.rs 或 src/front_of_house/hosting.rs
//
// Cargo.toml 需要声明这些模块
//
// 目录结构示例：
// src/
//   main.rs
//   lib.rs
//   front_of_house/
//     hosting.rs
//     serving.rs

// ========== write_to_file 函数 ==========
fn write_to_file() -> Result<(), io::Error> {
    let mut file = io::BufWriter::new(
        std::fs::File::create("output.txt")?
    );
    file.write_all(b"Hello, world!")?;
    Ok(())
}

// ============================================================================
// 模块最佳实践
// ============================================================================
//
// 1. 对于库，使用 lib.rs 作为模块根
// 2. 对于二进制，考虑将主逻辑移到 bin/ 或 src/lib.rs
// 3. 将大型模块拆分为多个文件
// 4. 使用 pub use 重新导出，方便用户使用
// 5. 保持模块树扁平，避免过深嵌套
//
// ============================================================================
// 模块可见性详解
// ============================================================================
//
// pub              - 公开，可从任何地方访问
// pub(crate)      - 仅在当前 crate 内可见
// pub(super)      - 仅在父模块中可见
// pub(in path)    - 仅在指定路径的模块中可见
//
// 结构体：
// - pub struct - 结构体本身公开
// - pub field  - 字段对外可见
// - private field - 只有模块内可访问
//
// 枚举：
// - pub enum - 枚举公开
// - pub variant - 所有变体默认公开（不能单独设置私有）
//
// ============================================================================
// 总结
// ============================================================================
//
// 1. crate 是 Rust 的编译单元
// 2. mod 定义模块，默认为私有
// 3. pub 控制可见性
// 4. use 将路径导入作用域
// 5. super 访问父模块
// 6. 嵌套模块可以使用文件层级组织
// 7. 结构体字段默认私有，枚举变体默认公开
//
