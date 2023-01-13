use std::thread;

mod channel_demo;
mod sharing_memory;

fn main() {
    simple();
    channel_demo::channel_demo();
}

fn simple() {
    let v = vec![1, 2, 3];
    // 这里必须要move，强制闭包获取v的所有权，否则v有可能在线程执行前就被释放了
    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
