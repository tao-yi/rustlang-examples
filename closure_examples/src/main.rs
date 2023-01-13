mod cache_example;

fn closures_example_1(num1: i32, num2: i32) -> i32 {
    let new_num = |x| {
        println!("Multiplying {} by 2", x);
        x * 2
    };

    if num1 > num2 {
        new_num(num1)
    } else {
        new_num(num2)
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", closures_example_1(3, 2));

    cache_example::generate_workout(10, 7);
    cache_example::generate_workout(12, 13);

    // 闭包可以捕获他们所在的环境，会产生内存开销
    let x = 4;
    let equal_to_x = |z| z == x;
    assert!(equal_to_x(4));

    // 函数则不能
    // fn equal_to_x(z: i32) -> bool {
    //     z == x // error
    // }

    fn_demo();
    fn_mut_demo();
    fn_once_demo();
}

/**
闭包从所在环境捕获值的方式
1. FnOnce: 取得值的所有权，消耗捕获的变量，因此只能被调用一次
2. FnMut: 获取值的可变引用
3. Fn: 获取值的不可变引用

创建闭包时，通过闭包对环境值的使用方式，rust会推断出具体使用哪一个trait
- 所有的闭包都实现了FnOnce
- 没有获取捕获变量的所有权的实现了 FnMut
- 没有获取可变引用的实现了 Fn
 */
fn fn_demo() {
    let x = vec![1, 2, 3];
    // 闭包取得一个不可变引用
    let equal_to_x = |z| z == x;
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    println!("{:?}", x);
}

fn fn_mut_demo() {
    let mut x = vec![1, 2, 3];
    // 闭包取得一个不可变引用
    let add_to_x = |z| {
        x.push(z);
        x
    };
    let y = vec![1, 2, 3];
    assert_eq!(vec![1, 2, 3, 4], add_to_x(4));
    // println!("{:?}", x);
}

fn fn_once_demo() {
    let x = vec![1, 2, 3];
    // 使用move强制闭包获取所有权
    let equal_to_x = move |z| z == x;
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    // println!("{:?}", x); // borrow of moved value error
}
