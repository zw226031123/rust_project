// ============================================================================
// Rust 学习笔记 08: 常见集合
// ============================================================================
//
// Rust 标准库提供了多种集合类型，最常用的有：
// - Vector<T> - 可变大小的动态数组
// - String - UTF-8 字符串（实际上是 Vec<u8> 的包装）
// - HashMap<K, V> - 键值对映射
// - HashSet<T> - 不重复值的集合
//
// ============================================================================

fn main() {
    // ==========================================================================
    // 1. Vector<T> - 动态数组
    // ==========================================================================
    println!("=== Vector 基础 ===");

    // 创建 Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);

    // 使用 vec! 宏创建
    let v2 = vec![1, 2, 3, 4, 5];
    println!("v2: {:?}", v2);

    // 读取元素
    println!("v2[0] = {}", v2[0]); // 索引访问，panics 如果越界
    println!("v2.get(0) = {:?}", v2.get(0)); // 返回 Option，不越界

    // 遍历 Vector
    println!("遍历 v2:");
    for i in &v2 {
        print!("{} ", i);
    }
    println!();

    // 遍历并修改（可变引用）
    let mut v3 = vec![1, 2, 3];
    for i in &mut v3 {
        *i += 10;
    }
    println!("修改后: {:?}", v3);

    // 枚举类型在 Vector 中的使用
    println!("\n=== Vector 存储枚举 ===");
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("整数: {}", i),
            SpreadsheetCell::Text(s) => println!("文本: {}", s),
            SpreadsheetCell::Float(f) => println!("浮点: {}", f),
        }
    }

    // ==========================================================================
    // 2. String - 字符串
    // ==========================================================================
    println!("\n=== String 基础 ===");

    // 创建 String
    let mut s = String::new();
    s.push('A');
    s.push_str("BC");
    println!("s: {}", s);

    // 从 &str 转换
    let s2 = String::from("hello");
    let s3 = s2 + " " + "world"; // s2 被移动，不能再使用
    println!("s3: {}", s3);

    // format! 宏（不获取所有权）
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let s7 = format!("{}-{}-{}", s4, s5, s6);
    println!("s7: {}", s7);

    // 字符串是 UTF-8
    let chinese = String::from("你好世界");
    println!("中文: {}", chinese);

    // 遍历字符串
    println!("遍历 'hello' 的字节:");
    for b in "hello".bytes() {
        print!("{} ", b);
    }
    println!();

    println!("遍历 '你好' 的字符:");
    for c in "你好".chars() {
        print!("{} ", c);
    }
    println!();

    // ==========================================================================
    // 3. HashMap<K, V> - 键值对映射
    // ==========================================================================
    println!("\n=== HashMap 基础 ===");

    use std::collections::HashMap;

    // 创建空的 HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    // 使用迭代器收集
    let teams = vec![String::from("Red"), String::from("Green")];
    let initial_scores = vec![30, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores2: {:?}", scores2);

    // 访问值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("{} 队的分数: {}", team_name, s),
        None => println!("没有找到 {} 队", team_name),
    }

    // 遍历 HashMap
    println!("遍历 scores:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新 HashMap
    println!("\n=== HashMap 更新 ===");

    // 插入（如果键不存在）
    scores.entry(String::from("Blue")).or_insert(100);
    scores.entry(String::from("Red")).or_insert(100);
    println!("插入后: {:?}", scores);

    // 基于旧值更新
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("词频统计: {:?}", word_count);

    // ==========================================================================
    // 4. HashSet<T> - 不重复集合
    // ==========================================================================
    println!("\n=== HashSet 基础 ===");

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(1); // 重复，不会添加

    println!("set: {:?}", set);
    println!("set 长度: {}", set.len());
    println!("set 包含 2 吗? {}", set.contains(&2));

    // HashSet 操作
    let set2: HashSet<_> = [3, 4, 5].iter().cloned().collect();
    println!("set2: {:?}", set2);

    // 并集
    let union: HashSet<_> = set.union(&set2).collect();
    println!("并集: {:?}", union);

    // 交集
    let intersection: HashSet<_> = set.intersection(&set2).collect();
    println!("交集: {:?}", intersection);

    // 差集
    let difference: HashSet<_> = set.difference(&set2).collect();
    println!("差集 (set - set2): {:?}", difference);

    // ==========================================================================
    // 5. 集合的常见操作
    // ==========================================================================
    println!("\n=== 集合高级操作 ===");

    // Vector 排序
    let mut numbers = vec![5, 2, 8, 1, 9];
    numbers.sort();
    println!("排序后: {:?}", numbers);

    // Vector 去重
    let mut with_dups = vec![1, 2, 2, 3, 3, 3, 4];
    with_dups.dedup();
    println!("去重后: {:?}", with_dups);

    // HashMap 查找并删除
    let removed = scores.remove(&String::from("Red"));
    println!("删除 Red: {:?}", removed);
    println!("删除后 scores: {:?}", scores);

    // ==========================================================================
    // 6. VecDeque - 双端队列
    // ==========================================================================
    println!("\n=== VecDeque (双端队列) ===");

    use std::collections::VecDeque;
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_front(1);
    deque.push_back(2);
    deque.push_front(0);
    deque.push_back(3);
    println!("deque: {:?}", deque);

    println!("pop_front: {:?}", deque.pop_front());
    println!("pop_back: {:?}", deque.pop_back());
    println!("剩余 deque: {:?}", deque);

    // ==========================================================================
    // 7. BTreeMap 和 BTreeSet - 有序集合
    // ==========================================================================
    println!("\n=== BTreeMap (有序映射) ===");

    use std::collections::BTreeMap;
    let mut btree = BTreeMap::new();
    btree.insert(3, "c");
    btree.insert(1, "a");
    btree.insert(2, "b");

    // BTreeMap 会按键排序
    for (k, v) in &btree {
        println!("{} -> {}", k, v);
    }

    // ==========================================================================
    // 8. 性能注意事项
    // ==========================================================================
    println!("\n=== 性能注意事项 ===");
    println!("1. Vec<T>: 栈上存储指针，堆上存储数据，索引 O(1)，尾部插入 O(1)");
    println!("2. String: 本质是 Vec<u8>，支持 UTF-8");
    println!("3. HashMap<K, V>: 平均 O(1) 查找/插入/删除");
    println!("4. HashSet<T>: 本质是 HashMap<T, ()>");
    println!("5. BTreeMap/Set: 有序，O(log n) 操作");
    println!("6. VecDeque: 双端 O(1) 插入/删除");
}

// ============================================================================
// 总结
// ============================================================================
//
// 1. Vec<T> - 动态数组，可变大小，索引 O(1)
// 2. String - UTF-8 字符串，是 Vec<u8> 的包装
// 3. HashMap<K, V> - 键值对映射，平均 O(1) 操作
// 4. HashSet<T> - 不重复集合，基于 HashMap 实现
// 5. VecDeque - 双端队列，两端 O(1) 操作
// 6. BTreeMap/Set - 有序版本，O(log n) 操作
// 7. entry().or_insert() 是更新 HashMap 的推荐方式
// 8. split_whitespace() 用于分割空白字符
// 9. 使用 HashSet 进行去重和集合运算
//
