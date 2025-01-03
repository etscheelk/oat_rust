//! Sparse vector entries.
//! 
//! 
//! An entry in a vector `v`, is a pair `(i, a)` such that `v[i] = a`.
//! There are many ways to store a vector entry in computer memory.  For example, you could store it
//! as a tuple `(i, a)`, as dictionary `{i: a}`, etc.  However, there are some operations that nearly everyone wants to perform on an entry,
//! no matter what data structure has been used to store it.  These operations are encoded in [traits](crate::tutorials::traits):
//! 
//! * [KeyValGet](crate::algebra::vectors::entries::KeyValGet) 
//!     allows a user to find the value of  `i` or `a`.  
//! 
//! * [KeyValSet](crate::algebra::vectors::entries::KeyValSet)
//!     allows a user to change the value of  `i` or  `a`.  
//! 
//! * [KeyValNew](crate::algebra::vectors::entries::KeyValNew)
//!     allows a user to create a new entry
//! 
//! **In OAT, a "vector entry" is considered to be any struct that implements one or more of these traits.**
//! 
//! # Example
//! 
//! ```
//! // Import the KeyValGet and KeyValSet traits, so that we can use them.
//! use oat_rust::algebra::vectors::entries::{KeyValGet, KeyValSet, KeyValNew}; 
//! 
//! // Define a vector entry.
//! // Every tuple of length 2 (that meets certain requirements) implements the KeyValGet and KeyValSet traits, automatically.
//! let mut vector_entry = (1, 0.);
//! 
//! // Use the methods associated with the `KeyValGet` trait: `.key()`, `.val()`
//! assert_eq!( vector_entry.key(), 1  ); // the .key() method retreives the index
//! assert_eq!( vector_entry.val(), 0. ); // the .val() method retreives the coefficient
//! 
//! // Use the methods associated with the `KeyValSet` trait: `.set_key()`, `.set_val()`
//! vector_entry.set_key( 2  );            // the .set_key() method sets the index
//! vector_entry.set_val( 3. );
//! assert_eq!( vector_entry.key(), 2   ); 
//! assert_eq!( vector_entry.val(), 3.  ); 
//! 
//! // Use the methods associated with `KeyValNew` trait: `new( key, val )`
//! // The general syntax takes form `T::new( key, val )`, where `T` is the type of the key-value pair.
//! // In this case, the key-value pair has type `( usize, f32 )`.  Rust uses pointed brackets to 
//! // express tuple-types, so the expression becomes ` <( usize, f32 )>::new( key, val )`
//! let vector_entry_new = <( usize, f32 )>::new( 2, 3. ); 
//! assert_eq!( vector_entry_new,  ( 2, 3. )  ); 
//! ```


use std::fmt;
use std::fmt::{Debug};
use std::marker::PhantomData;

use auto_impl::auto_impl;

use crate::utilities::functions::evaluate::EvaluateFunction;


//  ---------------------------------------------------------------------------
//  KEY-VALUE TRAIT -- ESTABLISHING TYPES
//  ---------------------------------------------------------------------------

/// A trait with no methods, used to eliminate type ambiguity.
pub trait KeyValTypes{
    type Key;
    type Val;
}

// Auto-implement for tuples of length 2.
// --------------------------------------

impl < Key, Val > KeyValTypes for (Key, Val) {
    type Key = Key; type Val = Val;
}


//  ---------------------------------------------------------------------------
//  KEY-VALUE TRAIT -- GETTING
//  ---------------------------------------------------------------------------


//  DESIGN NOTES
//
//  NOTE 1
//
//  There are least 3 things you might want to get from Val: the variable
//  itself, an immutable ref to the variable, and a mutable ref to the
//  variable.  There are circumstances that call for / exclude any of these
//  three:
//  1)  geting values from a tuple (tuples only let you take ref's out to their
//      values; if you want a non-ref, you might have to clone?)
//  2)  a function that spits out a new value (namely a value that's not stored
//      inside a struct and can't be reference).
//  Currently we're sticking with the simplest option.
//
//  NOTE 2
//
//  Deciding to make Key and Val associated types for the following reasons
//  1)  One runs into type problems (unconstrained types, unused types, etc.)
//      if one makes Key and Val into type parameters
//  2)  This trait is primarily intended to be implemented on the Items of
//      an iterator.  Since Item is uniquely determined by the iterator, and
//      the Key/Val types are uniquely determiend by the item, this doesn't
//      seem to impose any constraints that one would get implicitly if one
//      were using generics




/// Get the key or value of a `(key, val)` pair (no matter the underlying data structure).
#[auto_impl(&)] // this macro auto-implements the trait for immutable references to structs that implement the trait
pub trait KeyValGet< Key, Val >

{
    /// Get the key in the `(key, val)` pair.
    fn key( &self ) -> Key;

    /// Get the val in the `(key, val)` pair.    
    fn val( &self ) -> Val;
}


// Auto-implement for tuples of length 2.
// --------------------------------------

impl< Key, Val >

    KeyValGet < Key, Val >

    for 
    
    ( Key, Val )
    
    where
        Key: Clone, // this is basically required, since o/w have to implement copy
        Val: Clone  // this is basically required, since o/w have to implement copy
{
    fn key( &self ) -> Key { self.0.clone() }
    fn val( &self ) -> Val { self.1.clone() }
}


//  ---------------------------------------------------------------------------
//  KEY-VALUE TRAIT -- SETTTNG 
//  ---------------------------------------------------------------------------


/// Set the key or value of a `(key, val)` pair (no matter the underlying data structure).
#[auto_impl(&mut)] // this macro auto-implements the trait for mutable references to structs that implement the trait
pub trait KeyValSet< Key, Val > : KeyValGet< Key, Val >

{
    /// Set the key of a `(key, val)` pair (no matter the underlying data structure).
    fn set_key( &mut self, key: Key ) ;

    /// Set the value of a `(key, val)` pair (no matter the underlying data structure).
    fn set_val( &mut self, val: Val ) ;
}


//  Auto-implement for tuples of length 2.
//  --------------------------------------

impl< Key, Val >

    KeyValSet < Key, Val >
    
    for 
    
    ( Key, Val )
    
    where
        Key: Clone,
        Val: Clone
{
    fn set_key( &mut self, key: Key ) { self.0 = key }
    fn set_val( &mut self, val: Val ) { self.1 = val }
}


//  ---------------------------------------------------------------------------
//  KEY-VALUE TRAIT -- MAKING 
//  ---------------------------------------------------------------------------


/// Create a new key-value pair, with the desired data structure.
pub trait KeyValNew< Key, Val >

{
    /// Set the key of a `(key, val)` pair (no matter the underlying data structure).
    fn new( key: Key, val: Val ) -> Self ;

}


//  Auto-implement for tuples of length 2.
//  --------------------------------------

impl< Key, Val >

    KeyValNew < Key, Val >
    
    for 
    
    ( Key, Val )
    
{
    fn new( key: Key, val: Val ) -> Self { ( key, val ) }
}




//  ---------------------------------------------------------------------------
//  STRUCT REPRESENTING THE FUNCTION THAT EXTRACTS THE KEY FROM A KEY-VAL PAIR
//  ---------------------------------------------------------------------------

/// Represents the function that takes a key-value pair as input and returns the key as output.
/// 
/// This struct is specifically designed to implement the [`EvaluateFunction`](oat_rust::utilities::functions::evaluate::EvaluateFunction)
/// trait.
pub struct ExtractKey< Key, Val > { 
    phantom_key: PhantomData< Key >, 
    phantom_val: PhantomData< Val > 
}

impl < Key, Val > ExtractKey< Key, Val > {
    /// Create a new struct `ExtractKey< Key, Val >`
    pub fn new() -> ExtractKey< Key, Val > { ExtractKey{ phantom_key: PhantomData, phantom_val: PhantomData } }
}

impl < Key, Val, Entry > 

    EvaluateFunction
        < Entry, Key > for 

    ExtractKey
        < Key, Val >

    where
        Entry:  KeyValGet< Key, Val > 

{
    fn evaluate_function( &self, input: Entry ) -> Key {
        input.key()
    }
}        


//  ---------------------------------------------------------------------------
//  STRUCT REPRESENTING THE FUNCTION THAT EXTRACTS THE VAL FROM A KEY-VAL PAIR
//  ---------------------------------------------------------------------------

/// Represents the function that takes a key-value pair as input and returns the key as output.
/// 
/// This struct is specifically designed to implement the [`EvaluateFunction`](oat_rust::utilities::functions::evaluate::EvaluateFunction)
/// trait.
pub struct ExtractVal< Key, Val > { 
    phantom_key: PhantomData< Key >, 
    phantom_val: PhantomData< Val > 
}

impl < Key, Val > ExtractVal< Key, Val > {
    /// Create a new struct `ExtractKey< Key, Val >`
    pub fn new() -> ExtractVal< Key, Val > { ExtractVal{ phantom_key: PhantomData, phantom_val: PhantomData } }
}

impl < Key, Val, Entry > 

    EvaluateFunction
        < Entry, Val > for 

    ExtractVal
        < Key, Val >

    where
        Entry:  KeyValGet< Key, Val > 

{
    fn evaluate_function( &self, input: Entry ) -> Val {
        input.val()
    }
}     



//  ---------------------------------------------------------------------------
//  STRUCT REPRESENTING THE FUNCTION THAT FIRST CHANGES INDEX, THEN CHANGES ENTRY TYPE
//  ---------------------------------------------------------------------------


/// Struct representing a funciton that first changes an entry's index, then its entry type (concretely, the struct implements `EvaluateFunction< EntryOld, EntryNew >`).
pub struct ReindexEntry< EntryOld, EntryNew, IndexOld, IndexNew, Coefficient, FunctionIndexOldToIndexNew > {
    function_entry_old_to_entry_new:    FunctionIndexOldToIndexNew,
    phantom_entryold:                   PhantomData< EntryOld >,
    phantom_entrynew:                   PhantomData< EntryNew >,
    phantom_indexold:                   PhantomData< IndexOld >,
    phantom_indexnew:                   PhantomData< IndexNew >,    
    phantom_snzval:                     PhantomData< Coefficient >
}

//  Implement the struct
impl < EntryOld, EntryNew, IndexOld, IndexNew, Coefficient, FunctionIndexOldToIndexNew >

    ReindexEntry
        < EntryOld, EntryNew, IndexOld, IndexNew, Coefficient, FunctionIndexOldToIndexNew >
{
    pub fn new( function_entry_old_to_entry_new: FunctionIndexOldToIndexNew ) -> Self {
        ReindexEntry{ function_entry_old_to_entry_new, phantom_entrynew: PhantomData, phantom_entryold: PhantomData, phantom_indexnew: PhantomData, phantom_indexold: PhantomData, phantom_snzval: PhantomData }
    }
}

//  EvaluateFunction
impl < EntryOld, EntryNew, IndexOld, IndexNew, Coefficient, FunctionIndexOldToIndexNew >

    EvaluateFunction
        < EntryOld, EntryNew > for
    
    ReindexEntry
        < EntryOld, EntryNew, IndexOld, IndexNew, Coefficient, FunctionIndexOldToIndexNew >

    where
        EntryOld:                   KeyValGet< IndexOld, Coefficient >,
        EntryNew:                   KeyValNew< IndexNew, Coefficient >,
        FunctionIndexOldToIndexNew: EvaluateFunction< IndexOld, IndexNew >,
{
    fn evaluate_function(&self, input: EntryOld) -> EntryNew {
        let index_new           =   (self.function_entry_old_to_entry_new).evaluate_function( input.key() );
        EntryNew::new( index_new, input.val() )
    }
}





















//  ---------------------------------------------------------------------------
//  KEY-VALUE ITEM STRUCT
//  ---------------------------------------------------------------------------


/// Struct that encodes a key-value pair.
///
/// Preferred to a tuple `(key, val)`, since the latter may require 
/// [rewriting in memory](https://www.reddit.com/r/rust/comments/79ry4s/tuple_performance/), 
/// and also has memory overhead for length (or does it?  have to check).
#[derive( Clone )]
pub struct KeyValItem< Key, Val > 
   // where Key: Clone + Debug,
   //       Val: Clone + Debug
{   
    pub key: Key, 
    pub val: Val 
}


//  Custom implementaiton of debug
//  ------------------------------
impl < Key, Val >
    Debug for KeyValItem 
    < Key, Val > 

    where Key:  Debug,
          Val:  Debug

{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("P")
         .field(&self.key)
         .field(&self.val)
         .finish()
    }
}

//  Implement KeyValGet 
//  ------------------------------

impl< Key, Val >
    KeyValGet < Key, Val >
    for 
    KeyValItem< Key, Val > 
    where
        Key: Clone,
        Val: Clone
{
    fn key( &self ) -> Key { self.key.clone() }
    fn val( &self ) -> Val { self.val.clone() }
}

//  Implement KeyValSet
//  --------------------------------------

impl< Key, Val >
    KeyValSet < Key, Val >
    for 
    KeyValItem< Key, Val > 
    where
        Key: Clone,
        Val: Clone
{
    fn set_key( &mut self, key: Key ) { self.key = key }
    fn set_val( &mut self, val: Val ) { self.val = val }
}