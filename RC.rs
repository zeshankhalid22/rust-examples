use std::rc::Rc;

#[derive(Debug)]
enum List{
    Cons(i32,Rc<List>),
    Nil,
}
use crate::List::{Cons,Nil};


fn main(){
    let a = Rc::new(Cons(5, Rc::new(Cons(7,
                                         Rc::new(Cons(9,Rc::new(Nil)))))));
    println!("count After creating a {}",Rc::strong_count(&a));

    let b = Cons(3,Rc::clone(&a));  // reference to a's memory location
    println!("count After creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));

    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
