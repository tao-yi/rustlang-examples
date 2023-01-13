use default_example::*;

fn main() {
    println!("Hello, world!");
    // User must implement Default rait in order to use unwrap_or_default ()
    let a = User::new("testuser123".to_string()).unwrap_or_default();
    println!("{:?}", a);

    let post1 = Post::default();
    println!("{:?}", post1);

    let post2 = Post::new("example content".to_string());
    println!("{:?}", post2);
}
