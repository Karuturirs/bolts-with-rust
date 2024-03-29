

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    println!("Welcome to bolts-with-rust");

     let x = 5;
     let y = &x;

     assert_eq!(5, x);
     assert_eq!(5, *y);

}
