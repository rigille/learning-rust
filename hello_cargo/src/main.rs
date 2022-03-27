fn main() {
    let t = 0;
    fn test() -> i32 {
        t
    }
    println!("test: {}", test());
    println!("test: {}", test());
    println!("test: {}", test());
    println!("test: {}", test());
    println!("test: {}", test());
    println!("Hello, world!");
}
