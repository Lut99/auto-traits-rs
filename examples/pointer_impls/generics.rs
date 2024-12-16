//  GENERICS.rs
//    by Lut99
//
//  Created:
//    16 Dec 2024, 14:54:24
//  Last edited:
//    16 Dec 2024, 15:22:22
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

// Define some other trait, this time with a confused T
#[pointer_impls(T = U)]
trait Bars<T: ?Sized> {
    fn bar(&self) -> &T;
}


// Let's implement it for some object.
struct Foo;
impl Foos<str> for Foo {
    fn foo(&self) -> &str { "Hello, world!" }
}
impl Bars<str> for Foo {
    fn bar(&self) -> &str { "Goodbye, world!" }
}



fn hello_world<F: ?Sized + Display>(helloer: impl Foos<F>) {
    println!("{}", helloer.foo());
}

fn goodbye_world<F: ?Sized + Display>(helloer: impl Bars<F>) {
    println!("{}", helloer.bar());
}





/***** ENTRYPOINT *****/
fn main() {
    // This is always possible
    hello_world(Foo);
    goodbye_world(Foo);

    // However, this isn't possible without the `#[pointer_impls]`! (try it!)
    hello_world(&Foo);
    goodbye_world(&Foo);
    hello_world(&mut Foo);
    goodbye_world(&mut Foo);
    hello_world(Box::new(Foo));
    goodbye_world(Box::new(Foo));
    // ...
}
