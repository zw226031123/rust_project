// ============================================================================
// Rust 学习笔记 06: 结构体
// ============================================================================
//
// 结构体（Struct）允许你创建自定义数据类型，将多个相关值组合在一起。
// Rust 有三种结构体类型：
// 1. 具名结构体（Named Field Struct）
// 2. 元组结构体（Tuple Struct）
// 3. 单元结构体（Unit Struct）
//
// ============================================================================

fn main() {
    // ========== 1. 定义和使用结构体 ==========
    println!("=== 结构体基础 ===");

    // 创建 User 结构体实例
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
        sign_in_count: 1,
    };

    // 访问结构体字段
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
    println!("活跃状态: {}", user1.active);
    println!("登录次数: {}", user1.sign_in_count);

    // ========== 2. 可变结构体 ==========
    println!("\n=== 可变结构体 ===");

    let mut user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        active: false,
        sign_in_count: 0,
    };

    // 可以修改字段
    user2.email = String::from("bob.new@example.com");
    user2.active = true;
    user2.sign_in_count += 1;

    println!("更新后的用户: {:?}", user2);

    // ========== 3. 字段简写语法 ==========
    println!("\n=== 字段简写语法 ===");

    let username = String::from("charlie");
    let email = String::from("charlie@example.com");

    let user3 = User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    };
    // 相当于: username: username, email: email, ...

    println!("用户3: {} - {}", user3.username, user3.email);

    // ========== 4. 结构体更新语法 ==========
    println!("\n=== 结构体更新语法 ===");

    let user4 = User {
        username: String::from("dave"),
        email: String::from("dave@example.com"),
        ..user1 // 其余字段从 user1 复制
    };

    println!("用户4: {} - {}", user4.username, user4.email);
    println!("用户4 登录次数: {} (来自 user1)", user4.sign_in_count);

    // ========== 5. 元组结构体 ==========
    println!("\n=== 元组结构体 ===");

    // 元组结构体类似元组，但有自定义类型名
    struct Color(i32, i32, i32);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("黑色: RGB({}, {}, {})", black.0, black.1, black.2);
    println!("原点: ({}, {})", origin.0, origin.1);

    // ========== 6. 单元结构体 ==========
    println!("\n=== 单元结构体 ===");

    // 没有字段的结构体，类似于 () 类型
    struct AlwaysEqual;

    let _subject = AlwaysEqual;
    println!("单元结构体实例创建成功");

    // ========== 7. 结构体方法 ==========
    println!("\n=== 结构体方法 ===");

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("矩形面积: {}", rect.area());

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("rect 能容纳 rect2? {}", rect.can_hold(&rect2));

    let rect3 = Rectangle {
        width: 100,
        height: 50,
    };
    println!("rect 能容纳 rect3? {}", rect.can_hold(&rect3));

    // ========== 8. 关联函数 ==========
    println!("\n=== 关联函数 ===");

    // 使用 :: 调用关联函数（类似静态方法）
    let square = Rectangle::square(25);
    println!("正方形面积: {}", square.area());

    // ========== 9. 调试打印 ==========
    println!("\n=== 调试打印 ===");

    // 需要添加 #[derive(Debug)] 或手动实现 Debug
    println!("user1: {:?}", user1);
    println!("user1 (美化): {:#?}", user1);

    // ========== 10. 结构体与所有权 ==========
    println!("\n=== 结构体与所有权 ===");

    let name = String::from("eve");
    let user5 = build_user(name, String::from("eve@example.com"));
    // println!("{}", name); // 错误！name 的所有权已移动

    println!("用户5: {} - {}", user5.username, user5.email);
}

// ============================================================================
// 结构体定义
// ============================================================================

// 具名结构体
#[derive(Debug)] // 自动实现 Debug trait
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// Rectangle 结构体（用于方法示例）
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle 的方法实现
impl Rectangle {
    // &self 是 self: &Rectangle 的简写
    // 不能获取所有权（除非显式使用 self 而不是 &self）

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 使用 self 获取所有权的方法（不常用）
    #[allow(unused)]
    fn into_square(self) -> Rectangle {
        let size = self.width.min(self.height);
        Rectangle {
            width: size,
            height: size,
        }
    }

    // 使用 &mut self 修改自身
    #[allow(unused)]
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // 静态关联函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// ============================================================================
// 函数定义
// ============================================================================

fn build_user(username: String, email: String) -> User {
    User {
        username, // 字段简写
        email,    // 字段简写
        active: true,
        sign_in_count: 1,
    }
}

// ============================================================================
// 总结
// ============================================================================
//
// 1. 结构体使用 struct 关键字定义
// 2. 使用 {} 包裹字段列表
// 3. 字段简写允许 username, email 代替 username: username
// 4. 结构体更新语法 ..instance 可以复制其余字段
// 5. 元组结构体：类似元组但有类型名
// 6. 单元结构体：没有字段的特殊类型
// 7. 方法使用 impl 块定义，第一个参数是 &self
// 8. 关联函数使用 :: 调用，类似于静态方法
// 9. #[derive(Debug)] 自动实现调试输出
//
