fn main() {
    // 1 &str：字符串字面量的引用
    // 字符串字面量实际上存放在程序的只读数据段中，在程序运行时会被加载到内存中读取
    let s = "Hello Rust";
    let mut s1 = "Hello Go";

    s1 = "Hello Rust";
    println!("{}", s1);

    // 2 String：字符串切片的引用
    // String 通过指针指向存放在堆上的字符串

		// 有多种方式可以在堆上创建字符串
    // let s2 = String::new();         // 空字符串
    // let s2 = "Hello Rust".to_string();
    // let s2: String = "Hello Rust".into();
    // let s2 = String::from("Hello Rust");

    // 可以使用ptr、len、cap获取String的指针、长度和容量

    let cap = s2.capacity();
    let len = s2.len();
    let ptr = s2.as_ptr();

    println!("len {}", len);
    println!("cap {}", cap);
    println!("pointer {:?}", ptr);

    // 3 字符串切片
    // 字符串本质上一个u8序列，支持切片操作

    let s1 = String::from("Hello Rust");
    let s2 = "Hello Rust";

    let slice1 = &s1[0..5];
    let slice2 = &s2[6..10];

    println!("slice1: {}", slice1); // Hello
    println!("slice2: {}", slice2); // Rust
}