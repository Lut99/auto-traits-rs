//  CUSTOM POINTER.rs
//    by Lut99
//
//  Created:
//    16 Dec 2024, 12:02:31
//  Last edited:
//    16 Dec 2024, 14:12:13
//  Auto updated?
//    Yes
//
//  Description:
//!   Shows deriving a pointer impl for a custom pointer.
//

use std::ops::{Deref, DerefMut};

use auto_traits::pointer_impls;


/***** POINTER *****/
/// Something of ourselves that acts as a pointer!
struct Pointer1<T>(T);
impl<T> Deref for Pointer1<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<T> DerefMut for Pointer1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

/// Something else that acts like a pointer, but does not implement deref.
struct Pointer2<T>(T);





/***** TRAITS *****/
// Define some trait - with pointer implementations!
#[pointer_impls(unimpl *, impl Pointer1<_>, impl Pointer2<_> = &self.0)]
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
    hello_world(Pointer1(Foo));
    hello_world(Pointer2(Foo));
}
