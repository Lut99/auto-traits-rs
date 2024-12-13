//  SIMPLE.rs
//    by Lut99
//
//  Created:
//    13 Dec 2024, 14:44:49
//  Last edited:
//    13 Dec 2024, 14:51:34
//  Auto updated?
//    Yes
//
//  Description:
//!   Showcases the most straightforward uses of the
//!   `#[pointer_impls]`-macro.
//

use auto_traits::pointer_impls;


/***** TRAITS *****/
// Define some trait - with pointer implementations!
#[pointer_impls]
trait HelloWorld {
    fn hello_world(&self) -> &str;
}

// Let's implement it for some object.
struct Foo;
impl HelloWorld for Foo {
    fn hello_world(&self) -> &str { "Hello, world!" }
}



// This method is just to showcase for what `HelloWorld` is implemented
fn hello_world(helloer: impl HelloWorld) {
    println!("{}", helloer.hello_world());
}





/***** ENTRYPOINT *****/
fn main() {
    // This is always possible
    hello_world(Foo);

    // However, this isn't possible without the `#[pointer_impls]`! (try it!)
    hello_world(&Foo);
    hello_world(&mut Foo);
}
