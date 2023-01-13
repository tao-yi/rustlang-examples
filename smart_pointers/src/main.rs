use std::{ops::Deref, rc};

mod box_demo;
mod rc_demo;
mod refcell_demo;

/*
智能指针：通常使用struct实现，并且实现了 Deref 和 Drop 这两个trait
Deref trait: 允许智能指针struct的实例像引用一样使用 `*` 运算符
Drop trait: 允许你自定义当智能指针实例走出作用域时的代码
 */
fn main() {
    println!("Hello, world!");
    box_demo::box_demo();
    deref_demo();
    demo_deref_coercion();

    let f1 = File {
        name: String::from("hello1.txt"),
    };
    let f2 = File {
        name: String::from("hello2.txt"),
    };
    assert_ne!(f1.name, f2.name);
    std::mem::drop(f2); // 提前调用drop函数
    std::mem::drop(f1); // 提前调用drop函数

    rc_demo::rc_demo();
}

// 如果一个类型实现了Deref trait，我们可以自定义解引用运算符 * 的行为
fn deref_demo() {
    // 常规的引用也是一种指针
    let x = 5;
    let y = &x;
    assert_eq!(x, *y); // *y 表示解引用，拿到引用&x指针指向的值

    // 把Box<T>当做引用
    let y = Box::new(x);
    assert_eq!(x, *y); // 因为Box实现了Deref，所以我们可以对它解引用

    let b = MyBox::new(5);
    assert_eq!(5, *b); // 因为我们为MyBox实现了Deref
    let c = b.deref(); // 返回的是 &i32 类型
    assert_eq!(5, *c); // 对 &i32 进行解引用，得到 i32
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn demo_deref_coercion() {
    fn hello(name: &str) {
        println!("hello, {}", name);
    }
    let m = MyBox::new(String::from("Rust"));
    // 编译时完成了隐式解引用，没有额外性能开销
    hello(&m);
    // 因为deref coecrion，所以当我们取引用 &MyBox<String> 时，相当于 &String
}

// Drop Trait 自定义当值离开作用域时发生的行为
// 比如，文件，网络资源释放

struct File {
    name: String,
}

impl Drop for File {
    fn drop(&mut self) {
        // TODO: close file handle
        println!("closing files {}", self.name)
    }
}
