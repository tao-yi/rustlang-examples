/**
RefCell<T> 持有数据的唯一所有权，只能用于单线程。

借用规则规定：
在任何时间中，同时只能拥有一个不可变引用，或者是多个可变引用。

Box<T> 只有一个持有者，编译时检查
Rc<T> 允许多持有者，编译时检查
RefCell<T> 只有一个持有者，运行时检查

RefCell<T> 即使本身不可变，但仍然能修改其中存储的值
 */
pub fn refcell_demo() {}