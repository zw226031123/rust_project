// ============================================================================
// Rust 学习笔记 01: 变量与不可变性
// ============================================================================
//
// Rust 的变量默认是不可变的（immutable），这是 Rust 安全性的核心特性之一。
// 这一特性迫使我们在编写代码时更加谨慎，减少意外的修改导致的 bug。
//
// ============================================================================

fn main() {
    // ========== 1. 变量声明 ==========
    // 使用 `let` 关键字声明变量
    // Rust 会自动推断类型，大多数情况下不需要显式标注类型

    let x = 5; // 推断为 i32 类型
    println!("x 的值是: {}", x);

    // 声明时显式指定类型（可选）
    let y: i32 = 10;
    let z: f64 = 3.14;

    println!("y = {}, z = {}", y, z);

    // ========== 2. 不可变性 ==========
    // 变量默认不可变，尝试修改会编译错误

    let immut_val = 42;
    // immut_val = 50; // 错误！不能修改不可变变量
    // println!("{}", immut_val); // 编译失败

    println!("不可变变量 immut_val = {}", immut_val);

    // ========== 3. 可变性 ==========
    // 使用 `mut` 关键字使变量可变

    let mut mutable_val = 100;
    println!("修改前的 mutable_val: {}", mutable_val);
    mutable_val = 200; // 现在可以修改
    println!("修改后的 mutable_val: {}", mutable_val);

    // ========== 4. 变量遮蔽（Shadowing） ==========
    // 可以用相同的名称声明新变量来"遮蔽"旧变量
    // 与 `mut` 不同，遮蔽可以改变类型

    let shadowed = 1;
    println!("shadowed (第一次): {}", shadowed);

    let shadowed = shadowed + 1; // 新变量遮蔽旧变量
    println!("shadowed (第二次): {}", shadowed);

    // 改变类型（遮蔽允许这样做，mut 不允许）
    let spaces = "   ";          // &str 类型
    let spaces = spaces.len();   // 现在是 usize 类型
    println!("spaces 长度: {}", spaces);

    // ========== 5. 常量 ==========
    // 使用 `const` 关键字声明常量
    // - 必须标注类型
    // - 只能设置为常量表达式的值（编译时可计算）
    // - 不可变且不能使用 `mut`

    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);

    // ========== 6. 变量解构 ==========
    // 可以解构元组或结构体

    let tuple = (1, "hello", true);
    let (a, b, c) = tuple;
    println!("解构元组: a={}, b={}, c={}", a, b, c);

    // ========== 7. 下划线开头忽略未使用变量 ==========
    let _unused = 42; // 编译器不会警告未使用
    let y = 10;       // 如果不使用会警告

    // ========== 8. 重新绑定与变量遮蔽的区别 ==========

    // 方法1: 使用 mut（值可变，类型不变）
    let mut number = 5;
    number = number + 1; // 允许
    println!("mut number: {}", number);

    // 方法2: 使用遮蔽（可以改变类型）
    let number = "五"; // 完全不同的类型
    println!("shadowed number: {}", number);

    // ========== 总结 ==========
    // 1. 默认变量不可变，使用 `mut` 使其可变
    // 2. `let` 声明变量，`const` 声明常量
    // 3. 遮蔽（Shadowing）允许创建同名新变量，可改变类型
    // 4. 下划线开头可抑制未使用变量警告
}
