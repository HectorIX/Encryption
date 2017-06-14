mod aes256;
mod sha256;

fn main() {
    println!("sha256 value: {:?}", sha256::sha256("Hello World!".to_string()));
}
