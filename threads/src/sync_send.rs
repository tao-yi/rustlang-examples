/**
在Rust语言中有两个并发的概念：
std::marker::Sync
std::marker::Send 两个trait称作标签trait，因为他们没有定义任何方法

Send: 允许线程间转移所有权
Rust中几乎所有类型都实现了Send, 但是Rc<T>没有实现

Sync: 允许多线程同时访问
可以安全的被多个线程引用

如果T实现了Sync，那么相当于&T实现了Send
- T的引用可以被安全的传递给另一个线程

基础类型都实现了Sync
Mutex<T>也实现了Sync

手动实现Send和Sync是不安全的.
 */



