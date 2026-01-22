// ============================================================================
// Rust 学习笔记 10: 错误处理
// ============================================================================
//
// Rust 的错误处理分为两大类：
// 1. 可恢复错误 (Recoverable Errors) - 使用 Result<T, E>
// 2. 不可恢复错误 (Unrecoverable Errors) - 使用 panic!
//
// Rust 没有异常机制，而是通过返回值和处理机制来处理错误。
//
// ============================================================================

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

// ========== 1. panic! 不可恢复错误 ==========
// println!("=== panic! 不可恢复错误 ===");

// panic! 会打印错误消息并退出程序
// fn demo_panic() {
//     panic!("这是一个严重的错误！");
// }

// 线程 panic 会导致整个线程终止
// 使用 catch_unwind 可以捕获 panic（不推荐用于正常控制流）

// ========== 2. Result 枚举 ==========
// println!("=== Result 枚举 ===");

// Result<T, E> 定义在 std::result 模块中
// pub enum Result<T, E> {
//     Ok(T),      // 成功，包含值
//     Err(E),     // 失败，包含错误
// }

// 打开文件示例
#[allow(dead_code)]
fn open_file(path: &str) -> Result<File, io::Error> {
    let f = File::open(path)?;
    // ? 运算符：如果 Err 则返回错误，如果是 Ok 则解包
    Ok(f)
}

// 使用 match 处理 Result
#[allow(dead_code)]
fn match_result() {
    let f = File::open("test.txt");

    let _file = match f {
        Ok(file) => file,
        Err(error) => {
            println!("打开文件失败: {:?}", error);
            // 可以返回或使用默认值
            // std::process::exit(1); // 或者直接退出
            return;
        }
    };
}

// ========== 3. 传播错误 ==========
// println!("\n=== 传播错误 ===");

// 读取文件内容并返回 Result
#[allow(dead_code)]
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?; // ? 运算符传播错误
    let mut content = String::new();
    f.read_to_string(&mut content)?; // ? 运算符传播错误
    Ok(content)
}

// 使用 match 手动传播
#[allow(dead_code)]
fn read_file_manual(path: &str) -> Result<String, io::Error> {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e), // 手动返回错误
    };

    let mut content = String::new();
    match f.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

// ========== 4. ? 运算符详解 ==========
// println!("\n=== ? 运算符 ===");

// ? 运算符的行为：
// 1. 如果是 Ok(value)，解包并返回 value
// 2. 如果是 Err(error)，从函数返回 Err(error)
// 3. 自动进行错误类型转换（使用 From trait）
#[allow(dead_code)]
fn chain_operations() -> Result<String, io::Error> {
    let mut file = File::open("config.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// ========== 5. 错误类型转换 ==========
// println!("\n=== 错误类型转换 (From trait) ===");

// ? 运算符会自动调用 From::from 转换错误类型
// 这允许从具体错误类型转换为泛型错误类型
#[allow(dead_code)]
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    Ok(s.parse::<i32>()?) // ParseIntError -> 具体错误类型
}

// 可以返回多种错误类型的组合
use std::error::Error;
use std::fmt;
#[allow(dead_code)]
#[derive(Debug)]
enum MyError {
    IoError(io::Error),
    ParseError(ParseIntError),
}

// 实现 Display trait
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO error: {}", e),
            MyError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

// 实现 Error trait
impl Error for MyError {}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        MyError::IoError(e)
    }
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::ParseError(e)
    }
}
#[allow(dead_code)]
fn parse_with_custom_error(s: &str) -> Result<i32, MyError> {
    let num: i32 = s.parse()?;
    Ok(num)
}

// ========== 6. unwrap 和 expect ==========
// println!("\n=== unwrap 和 expect ===");
#[allow(unused)]
fn demo_unwrap() {
    // unwrap: 如果是 Ok 返回值，如果是 Err 则 panic
    let f = File::open("test.txt").unwrap(); // 可能 panic

    // expect: 类似 unwar，但可以自定义错误消息
    let f2 = File::open("test.txt").expect("无法打开配置文件"); // panic 时显示此消息
}

// ========== 7. 多种错误处理模式 ==========
// println!("\n=== 错误处理模式 ===");

// 模式1: match 表达式
#[allow(dead_code)]
fn handle_with_match(result: Result<i32, &str>) -> i32 {
    match result {
        Ok(value) => value,
        Err(e) => {
            eprintln!("错误: {}", e);
            -1 // 返回默认值
        }
    }
}

// 模式2: map 转换错误
#[allow(dead_code)]
fn parse_and_double(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map(|n| n * 2).map_err(|e| e.to_string())
}

// 模式3: and_then 链式处理
#[allow(dead_code)]
fn parse_and_add(s1: &str, s2: &str) -> Result<i32, String> {
    s1.parse::<i32>()
        .and_then(|n1| s2.parse::<i32>().map(|n2| n1 + n2))
        .map_err(|e| e.to_string())
}

// 模式4: or_else 自定义错误
#[allow(dead_code)]
fn read_config() -> Result<String, io::Error> {
    let mut f = File::open("config.txt").or_else(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            File::open("default_config.txt")
        } else {
            Err(e)
        }
    })?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

// 模式5: unwrap_or 和 unwrap_or_else
#[allow(dead_code)]
fn get_value(result: Result<i32, &str>) -> i32 {
    result.unwrap_or(0) // Err 时返回 0
    // result.unwrap_or_else(|e| { 0 }) // 使用闭包计算默认值
}

// ========== 8. 创建自定义错误类型 ==========
// println!("\n=== 自定义错误类型 ===");

use thiserror::Error; // 需要添加 thiserror 依赖

#[derive(Error, Debug)]
pub enum AppError {
    #[error("文件操作错误: {source}")]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("解析错误: {source}")]
    Parse {
        #[from]
        source: ParseIntError,
    },

    #[error("验证失败: {message}")]
    Validation { message: String },
}
#[allow(dead_code)]
fn validate_age(age: &str) -> Result<i32, AppError> {
    let age: i32 = age.parse()?;
    if age < 0 || age > 150 {
        return Err(AppError::Validation {
            message: format!("年龄 {} 无效", age),
        });
    }
    Ok(age)
}

// ========== 9. 与 panic 的选择 ==========
// println!("\n=== 何时使用 panic vs Result ===");

// 使用 panic 的情况：
// 1. 原型代码
// 2. 测试代码
// 3. 不可恢复的错误（如约定被违反）
// 4. 使用 unwrap/expect 时

// 使用 Result 的情况：
// 1. 可能失败的操作（I/O、网络等）
// 2. 需要让调用者决定如何处理错误
// 3. 库代码中

// ========== 10. 综合示例 ==========
// println!("\n=== 综合示例 ===");

fn process_user_input(input: &str) -> Result<i32, Box<dyn Error>> {
    // 使用 Box<dyn Error> 接受任何错误类型
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "输入为空",
        )));
    }

    let num: i32 = trimmed.parse()?;
    if num < 0 {
        return Err("数字不能为负".into());
    }

    Ok(num)
}

fn main() {
    // 实际测试
    println!("\n实际测试：");

    // 测试 Result 处理
    let results = vec![Ok(42), Err("错误信息"), Ok(100)];

    for r in results {
        match r {
            Ok(v) => println!("  成功: {}", v),
            Err(e) => println!("  失败: {}", e),
        }
    }

    // 测试 parse
    let test_cases = vec!["42", "abc", "-5", ""];
    for s in test_cases {
        match process_user_input(s) {
            Ok(n) => println!("  '{}' -> {}", s, n),
            Err(e) => println!("  '{}' -> 错误: {}", s, e),
        }
    }
}

// ============================================================================
// 总结
// ============================================================================
//
// 1. panic! 用于不可恢复的错误，会终止程序
// 2. Result<T, E> 用于可恢复的错误
// 3. match 是处理 Result 的基本方式
// 4. ? 运算符简化错误传播
// 5. unwrap/expect 方便但可能导致 panic
// 6. map, and_then 等组合器用于链式处理
// 7. 自定义错误类型提高代码可读性
// 8. Box<dyn Error> 用于泛化错误类型
// 9. 库代码应返回 Result，应用代码可以选择处理或 panic
//
