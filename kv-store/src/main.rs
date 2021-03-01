mod protocol;

pub type Err = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Err>;

fn main() {
    println!("Hello, world!");
}
