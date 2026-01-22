// ============================================================================
// Rust 学习笔记 02: 数据类型
// ============================================================================
//
// Rust 是静态类型语言，编译时必须知道所有变量的类型。
// 类型系统是 Rust 安全性的基础，能够在编译时捕获许多错误。
//
// ============================================================================

fn main() {
    // ========== 1. 标量类型 (Scalar Types) ==========
    // 标量类型表示单个值，Rust 有四种标量类型：整数、浮点数、布尔值、字符

    // ---------- 整数类型 ----------
    // 有符号整数: i8, i16, i32, i64, i128, isize
    // 无符号整数: u8, u16, u32, u64, u128, usize

    let decimal = 98_222; // 十进制
    let hex = 0xff; // 十六进制
    let octal = 0o77; // 八进制
    let binary = 0b1111_0000; // 二进制
    let byte = b'A'; // 字节（u8 的别名）

    println!(
        "整数示例: decimal={}, hex={}, octal={}, binary={}, byte={}",
        decimal, hex, octal, binary, byte
    );

    // isize 和 usize 依赖于计算机架构
    // 64 位系统上是 64 位，32 位系统上是 32 位
    let arch_specific: usize = 42;
    println!("usize 大小: {} 字节", std::mem::size_of_val(&arch_specific));

    // ---------- 浮点数类型 ----------
    // f32（单精度）和 f64（双精度）
    // 默认类型是 f64

    let x = 2.0; // f64
    let y: f32 = 3.0; // 显式指定 f32

    println!("浮点数: x={}, y={}", x, y);

    // 浮点数的特殊值
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    let nan = f64::NAN;

    println!("无穷大: {}, 负无穷: {}, NaN: {}", inf, neg_inf, nan);

    // ---------- 布尔类型 ----------
    // bool 类型只有两个值: true 和 false
    // 占 1 个字节

    let t = true;
    let f: bool = false; // 显式指定类型

    println!("布尔值: t={}, f={}", t, f);

    // 布尔值常用在条件表达式中
    let condition = 10 > 5;
    println!("10 > 5 的结果是: {}", condition);

    // ---------- 字符类型 ----------
    // char 类型表示单个 Unicode 字符
    // 使用单引号，4 个字节（可以表示中文、emoji 等）

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("字符示例: {}, {}, {}", c, z, heart_eyed_cat);

    // ========== 2. 复合类型 (Compound Types) ==========
    // 复合类型可以将多个值组合成一个类型

    // ---------- 元组 (Tuple) ----------
    // 元组是固定长度的、有序的、异质的（可以包含不同类型）
    // 创建后长度不能改变

    let tuple: (i32, f64, &str, bool) = (500, 6.4, "hello", true);

    // 访问元组元素
    println!("元组第一个元素: {}", tuple.0);
    println!("元组第二个元素: {}", tuple.1);

    // 解构元组
    let (a, b, c, d) = tuple;
    println!("解构: a={}, b={}, c={}, d={}", a, b, c, d);

    // 空元组 ()
    let _empty_tuple: () = ();
    println!("空元组");

    // ---------- 数组 (Array) ----------
    // 数组是固定长度的、同质的（所有元素类型相同）
    // 数组在栈上分配，固定大小

    let array = [1, 2, 3, 4, 5];

    // 访问数组元素
    println!("数组第一个元素: {}", array[0]);
    println!("数组长度: {}", array.len());

    // 创建重复值的数组
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("重复数组: {:?}", zeros);

    // 显式指定类型和长度
    let floats: [f64; 3] = [1.1, 2.2, 3.3];
    println!("浮点数组: {:?}", floats);

    // 数组越界访问会导致 panic（运行时错误）
    // println!("{}", array[10]); // 危险！会导致 panic

    // ========== 3. 类型转换 ==========
    // Rust 不会自动进行类型转换，需要显式转换

    let int_val: i32 = 42;
    // let float_val: f64 = int_val; // 错误！不能隐式转换
    let float_val: f64 = int_val as f64; // 使用 as 进行转换
    println!("显式转换: {} -> {}", int_val, float_val);

    // ========== 4. 类型推断 ==========
    // 编译器可以推断变量类型

    let inferred_val = 42; // 推断为 i32
    let inferred_float = 3.14; // 推断为 f64

    println!(
        "推断类型: {} 是 i32, {} 是 f64",
        inferred_val, inferred_float
    );

    // ========== 5. 类型别名 ==========
    // 使用 type 关键字创建类型别名

    type Kilometers = i32;

    let distance: Kilometers = 100;
    let distance2: Kilometers = distance + 1;
    println!("距离: {} 公里", distance2);

    // ========== 6. never 类型 (!) ==========
    // never 类型表示永远不会返回（用于发散函数）
    // 例如: panic!(), continue, loop

    // 这是一个发散函数（从不返回）
    #[allow(dead_code)]
    fn diverge() -> ! {
        panic!("这个函数永远不会返回！");
    }

    // 不能调用这个函数，会导致 panic
    // diverge();

    // ========== 总结 ==========
    // 1. 标量类型: 整数(i/u + 位数)、浮点数(f32/f64)、布尔(bool)、字符(char)
    // 2. 复合类型: 元组(固定长度、异质)、数组(固定长度、同质)
    // 3. 类型转换需要显式使用 as
    // 4. 编译器会尽可能推断类型
}
