// Rust 程序入口函数，跟其它语言一样，都是 main，该函数目前无返回值
fn main() {
    let guess: i32 = "42".parse().expect("Not a number!");
    println!("{}", guess);
}
