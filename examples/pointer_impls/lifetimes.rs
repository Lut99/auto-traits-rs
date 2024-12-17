//  LIFETIMES.rs
//    by Lut99
//
//  Created:
//    17 Dec 2024, 15:58:26
//  Last edited:
//    17 Dec 2024, 16:41:17
//  Auto updated?
//    Yes
//
//  Description:
//!   TODO
//

use auto_traits::pointer_impls;


/***** TRAITS *****/
// Define some trait - with pointer implementations!
#[pointer_impls]
trait Passer {
    fn pass<'s>(what: &'s str) -> &'s str;
}

// Let's implement it for some object.
struct Foo;
impl Passer for Foo {
    fn pass<'s>(what: &'s str) -> &'s str { what }
}



// This method is just to showcase for what `HelloWorld` is implemented
fn pass<P: Passer>(what: &str) -> &str { P::pass(what) }





/***** ENTRYPOINT *****/
fn main() {
    // This is always possible
    println!("{}", pass::<Foo>("Hello, world!"));

    // However, this isn't possible without the `#[pointer_impls]`! (try it!)
    println!("{}", pass::<&Foo>("Hello, world!"));
    println!("{}", pass::<&mut Foo>("Hello, world!"));
    println!("{}", pass::<Box<Foo>>("Hello, world!"));
    // ...
}
