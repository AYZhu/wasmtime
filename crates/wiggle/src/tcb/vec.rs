use std::{
    alloc::{Allocator, Global},
};


use flux_rs::attrs::*;
use flux_core::*;

//---------------------------------------------------------------------------------------
#[extern_spec]
#[refined_by(cap: int)]
#[invariant(0 <= cap)]
struct Vec<T, A: Allocator = Global>;

#[extern_spec]
impl<T> Vec<T> {
    #[flux::sig(fn() -> Vec<T>[0])]
    fn new() -> Vec<T>;
    
    #[flux::sig(fn(n: usize) -> Vec<T>[n])]
    fn with_capacity(n: usize) -> Vec<T>;
}

// AYZTODO: This is lying a bit, I think. base == addr does not seem like it is
//          necessarily true, but it is perhaps okay to model a vector as an
//          array.

#[extern_spec]
impl<T, A: Allocator> Vec<T, A> {
    #[flux::sig(fn(&mut Vec<T, A>[@n]) ->
        {base:int, addr:int . 
            *mut[base, addr, n * T::size_of()] T
        | base >= 0 && base % T::align_of() == 0 && base == addr && addr != 0})]
    fn as_mut_ptr(&mut self) -> *mut T;
}