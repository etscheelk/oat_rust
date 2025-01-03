//! Tips for clones, copies, and references
//! 
//! This page is meant more as a reference than an introduction to clones, copies, and references. Check out the [Rust book](https://doc.rust-lang.org/book/ch00-00-introduction.html) for help getting started.
//! 
//! # Clone and copy
//! 
//! #### Clone and copy aren't always deep
//! 
//! `Clone` and `Copy` sometimes do and sometimes don't yield deep copies of a type `T` -- it depends on `T`!
//! 
//! #### Clone and copy on references
//! 
//! The official [Rust docs](https://doc.rust-lang.org/stable/reference/types/pointer.html?highlight=reborrow) describe how reference clone and copy.
//! 
//! - Immutable references `&T`
//!   - Implement `Copy`, provided that `T` implements the `Sized` trait.  The implementation is shallow, in the sense that only the pointer copies, not the data pointed to.
//!   - Calling `.clone()` on an immutable reference `&T` **does not clone the pointer**.  Instead it returns a cloned copy of the referenced object, not a clone of the reference.  In particular, it will return an object of type `T`.
//! - Mutable references `&mut T`
//!   - Never implement `Copy` (because only one mutable reference to an object can exist at one time)
//! 
//! #### Deriving
//! 
//! - `#[derive(Clone)]` can be used to automatically implement `Clone` on a struct.  The macro will check that all the types associated with the struct implement Clone (possibly even types that aren't associated with any field); then it will implement a field-wise clone, c.f. [this website](https://stegosaurusdormant.com/understanding-derive-clone/)
//! 
//! - the condition that every generic parameter associated with a struct must implement `Clone` can be overly restrictive.  If `#[derive(Clone)]` fails for this reason, you can always manually implement `Clone`
//! 
//! - below is an example  ([source](https://hashrust.com/blog/moves-copies-and-clones-in-rust/))
//! 
//!   ```compile_fail
//!   //error:the trait `Copy` may not be implemented for this type
//!   //because its nums field does not implement `Copy`
//!   #[derive(Copy, Clone)]
//!   struct Numbers {
//!       nums: Vec<i32>
//!   }
//!   ```
//! 
//! #### How to implement `Copy`
//! 
//! You can implement `Copy`  manually as shown below ([source](https://hashrust.com/blog/moves-copies-and-clones-in-rust/)), but **there are no methods** in the `Copy` trait; you have to let Rust figure out everything.  Your struct must also meet certain requirements.
//! 
//!   ```
//!   struct Point {
//!       x: i32,
//!       y: i32,
//!   }
//! 
//!   //no method in Copy because it is a marker trait
//!   impl Copy for Point {}
//! 
//!   impl Clone for Point {
//!       fn clone(&self) -> Point {
//!           *self
//!       }
//!   }
//!   ```
//! 
//! # References
//! 
//! #### Lifetime annotations describe the referenced object, not the reference itself
//! 
//! Consider the following example.  The first function delcaration does not result in an error
//! because the reference `& self.worm` points at an object with lifetime `'a`.  The second function declaration 
//! results in an error because `&'a self.worm` *points* at something with lifetime `'a`, but does not have
//! lifetime `'a` itself.
//! 
//! ```
//! // an apple containing a worm
//! pub struct AppleVal{ 
//!     worm: usize
//! }
//! 
//! impl AppleVal {  
//! 
//!     fn worm_ref< 'a >( &'a self ) -> &'a usize { & self.worm }
//! 
//!     // fn worm_ref_ref< 'a >( &'a self ) -> &'a &'a usize { && self.worm }
//! }
//! ```
//! 
//! A similar phenomenon can be found below.  Here only the third function declaration,
//! `worm_ref_ref_ref`, results in an error.
//! 
//! ```
//! // apple containing reference
//! pub struct AppleRef<'a>{ 
//!     worm: &'a usize
//! }
//! 
//! impl <'a> AppleRef < 'a > {  
//! 
//!     fn worm_ref( &'a self) -> &'a usize { self.worm }
//! 
//!     fn worm_ref_ref( &'a self ) -> &'a &'a usize { & self.worm }
//! 
//!     // fn worm_ref_ref_ref( &'a self ) -> &'a &'a &'a usize { && self.worm }    
//! }
//! ```
