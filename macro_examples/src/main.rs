use macro_examples;
use std::collections::HashMap;

#[macro_export]
macro_rules! make_map {
    ( $($k:expr, $v:expr),* ) => {{
        let mut m = HashMap::new();
        $(
            println!("Key: {}", $k);
            println!("Value: {}", $v);
            m.insert($k, $v);
        )*
        m
    }};
}

fn main() {
    println!("Hello, world!");

    let b = macro_examples::vec!(1, 2, 3);
    println!("{:?}", b);

    let b = macro_examples::vec!("1", "2", "3");
    println!("{:?}", b);

    let int_map = make_map!(1, 3);
    println!("{:?}", int_map);

    let str_map = make_map!("1", "3", "2", "4", "5", "6");
    println!("{:?}", str_map);
}
