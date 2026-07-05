//use std::ops::Deref;
//
//struct MyBox<T>(T);
//
//impl<T> MyBox<T> {
//    fn new(x: T) -> MyBox<T> {
//        MyBox(x)
//    }
//}
//
//impl<T> Deref for MyBox<T> {
//    type Target = T;
//
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}
//struct CustomSmartPointer {
//    data: String,
//}
//
//impl Drop for CustomSmartPointer {
//    fn drop(&mut self) {
//        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//    }
//}
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::cell::{Ref, RefCell};
use std::rc::Rc;
fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
    //    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //    println!("count after creating a = {}", Rc::strong_count(&a));
    //    let b = Cons(2, Rc::clone(&a));
    //    println!("count after creating b = {}", Rc::strong_count(&a));
    //    {
    //        let c = Cons(4, Rc::clone(&a));
    //        println!("count after creating c = {}", Rc::strong_count(&a));
    //    }
    //    println!(
    //        "count after c goee put of the scope = {}",
    //        Rc::strong_count(&a)
    //    );
    //  let x = 5;
    //  let y = MyBox::new(x);
    //
    //  assert_eq!(5, x);
    //  assert_eq!(5, *y);
    //
    //  let m = MyBox::new(String::from("Rust"));
    //  hello(&m);

    //    let c = CustomSmartPointer {
    //        data: String::from("my stuff"),
    //    };
    //    println!("CustomSmartPointer created");
    //    drop(c);
    //    println!("CustomSmartPointer droped before the end of main");
    //}

    //fn hello(name: &str) {
    //    println!("Hello, {name}");
}
