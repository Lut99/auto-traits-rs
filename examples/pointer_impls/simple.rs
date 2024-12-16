//  SIMPLE.rs
//    by Lut99
//
//  Created:
//    13 Dec 2024, 14:44:49
//  Last edited:
//    16 Dec 2024, 15:09:45
//  Auto updated?
//    Yes
//
//  Description:
//!   Showcases the most straightforward uses of the
//!   `#[pointer_impls]`-macro.
//

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

use auto_traits::pointer_impls;


/***** TRAITS *****/
// Define some trait - with pointer implementations!
#[pointer_impls]
trait HelloWorld {
    fn hello(&self, what: &str) -> String;
}

// Let's implement it for some object.
struct Foo;
impl HelloWorld for Foo {
    fn hello(&self, what: &str) -> String { format!("Hello, {what}!") }
}



// This method is just to showcase for what `HelloWorld` is implemented
fn hello_world(helloer: impl HelloWorld) {
    println!("{}", helloer.hello("world"));
}





/***** ENTRYPOINT *****/
fn main() {
    // This is always possible
    hello_world(Foo);

    // However, this isn't possible without the `#[pointer_impls]`! (try it!)
    hello_world(&Foo);
    hello_world(&mut Foo);
    hello_world(Box::new(Foo));
    hello_world(Rc::new(Foo));
    hello_world(Arc::new(Foo));
    hello_world(RefCell::new(Foo).borrow());
    hello_world(RefCell::new(Foo).borrow_mut());
    hello_world(Mutex::new(Foo).lock().unwrap());
    hello_world(RwLock::new(Foo).read().unwrap());
    hello_world(RwLock::new(Foo).write().unwrap());
    #[cfg(feature = "parking_lot")]
    hello_world(parking_lot::Mutex::new(Foo).lock());
    #[cfg(feature = "parking_lot")]
    hello_world(parking_lot::RwLock::new(Foo).read());
    #[cfg(feature = "parking_lot")]
    hello_world(parking_lot::RwLock::new(Foo).write());
}
