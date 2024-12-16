//  GENERICS.rs
//    by Lut99
//
//  Created:
//    16 Dec 2024, 14:54:24
//  Last edited:
//    16 Dec 2024, 14:55:59
//  Auto updated?
//    Yes
//
//  Description:
//!   A file showcasing the macro works with generics.
//

use std::fmt::Display;

use auto_traits::pointer_impls;


/***** TRAITS *****/
// Define some trait - with pointer implementations!
#[pointer_impls]
trait Foos<F: ?Sized> {
    fn foo(&self) -> &F;
}

// Let's implement it for some object.
struct Foo;
impl Foos<str> for Foo {
    fn foo(&self) -> &str { "Hello, world!" }
}



// This method is just to showcase for what `HelloWorld` is implemented
fn hello_world<F: ?Sized + Display>(helloer: impl Foos<F>) {
    println!("{}", helloer.foo());
}





/***** ENTRYPOINT *****/
fn main() {
    // This is always possible
    hello_world(Foo);

    // However, this isn't possible without the `#[pointer_impls]`! (try it!)
    hello_world(&Foo);
    hello_world(&mut Foo);
    hello_world(Box::new(Foo));
    // ...
}
