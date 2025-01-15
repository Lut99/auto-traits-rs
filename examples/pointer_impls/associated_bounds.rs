//  ASSOCIATED BOUNDS.rs
//    by Lut99
//
//  Created:
//    16 Dec 2024, 14:27:33
//  Last edited:
//    15 Jan 2025, 10:43:56
//  Auto updated?
//    Yes
//
//  Description:
//!   Shows that the `pointer_impls`-macro also works for associated
//!   types with bounds on their associated types.
//

use std::fmt::Display;

use auto_traits::pointer_impls;


/***** TRAITS *****/
// Define some trait - with pointer implementations!
#[pointer_impls]
trait Foos {
    type Foo: ?Sized;

    fn foo(&self) -> &Self::Foo;
}
#[pointer_impls]
trait DisplayFoo: Foos
where
    Self::Foo: Display,
{
}

// Let's implement it for some object.
struct Foo;
impl Foos for Foo {
    type Foo = str;

    fn foo(&self) -> &Self::Foo { "Hello, world!" }
}
impl DisplayFoo for Foo {}



// This method is just to showcase for what `HelloWorld` is implemented
fn hello_world<F>(foo: F)
where
    F: DisplayFoo,
    F::Foo: Display,
{
    println!("{}", foo.foo());
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
