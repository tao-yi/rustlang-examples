/**
Rc<T> 引用计数只能指针
有时候一个值会有多个所有者：有多个值拥有它的所有权。
使用场景：
需要在heap上分配数据，这些数据被程序的多个部分读取（只读），
但在编译时无法确定哪个部分最后使用完这些数据

Rc<T>会在实例内部维护一个引用计数，是线程不安全的。
只能在单线程使用。
 */

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use crate::rc_demo::List::{Cons, Nil};
pub fn rc_demo() {
    // 5 -> 10 -> nil
    let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));
    // 使用Rc来让a可以被多个变量共享所有权
    let a = Rc::new(a);
    {
        // 调用clone会使引用计数+1
        // 3 -> 5 -> 10 -> nil
        let b = Cons(3, Rc::clone(&a));
        // 4 -> 5 -> 10 -> nil
        let c = Cons(4, Rc::clone(&a));
        println!("rc weak count:{}", Rc::weak_count(&a));
        println!("rc strong count: {}", Rc::strong_count(&a));
    }
    println!("weak count after leave scope {}", Rc::weak_count(&a));
    println!("strong count after leave scope {}", Rc::strong_count(&a));
}
