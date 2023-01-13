// 使用Mutex来保证每一次只允许一个线程来访问数据

use std::{
    sync::{Arc, Mutex},
    thread,
};

fn demo() {
    let counter = Mutex::new(0);
    // counter需要被每一个线程拥有所有权，使用Atomic Rc, Arc类型确保线程安全
    let counter = Arc::new(counter);
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        demo()
    }
}
