pub fn box_demo() {
    // Box<T> 允许你在heap上存储数据，而在stack上面存放指向该数据的指针
    // Box实现了Deref和Drop trait
    // 场景：在编译时某个类型的大小无法确定
    // 但你有大量数据，想移交所有权，但是需要确保数据不被复制
    let b = Box::new(5);
    println!("b = {}", b);
    // 当b离开作用域时，会释放stack上的指针，和heap中的数据

    use crate::box_demo::List::{Cons, Nil};
    // recursive struct
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    list.traverse();
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn traverse(&self) {
        let mut cur = self;
        let mut res = String::new();
        loop {
            match cur {
                List::Cons(num, next) => {
                    res.push_str(&num.to_string());
                    res.push_str("->");
                    cur = next
                }
                List::Nil => {
                    res.push_str("nil");
                    break;
                }
            }
        }
        println!("{}", res)
    }
}
