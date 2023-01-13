use std::{sync::mpsc, thread, time::Duration};

/**
mpsc::channel() 创建channel
mpsc: multiple producer single consumer 多个生产者，一个消费者
 */
pub fn channel_demo() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            // send方法会拿到所有权
            sender.send(val).unwrap();
            // println!("val is {}", val); // cannot borrow moved value
            thread::sleep(Duration::from_millis(1))
        }
    });

    // recv会阻塞线程，直到从channel中收到消息
    // let received = receiver.recv().unwrap();
    // // try_recv 方法不会阻塞：通常是用loop循环检查channel是否有数据
    // println!("Got {}", received);

    // 可以把chanel当做一个迭代器使用
    for received in receiver {
        println!("Got: {}", received);
    }
}

fn multiple_senders() {
    let (sender, received) = mpsc::channel();
    let sender2 = mpsc::Sender::clone(&sender);

    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi"),
            String::from("1: from"),
            String::from("1: the"),
            String::from("1: thread"),
        ];
        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2: hi"),
            String::from("2: from"),
            String::from("2: the"),
            String::from("2: thread"),
        ];
        for val in vals {
            sender2.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for msg in received {
        println!("Got: {}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_channel_demo() {
        channel_demo();
    }

    #[test]
    fn test_multiple_senders() {
        multiple_senders();
    }
}
