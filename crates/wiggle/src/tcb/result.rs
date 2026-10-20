//
// TODO (alz034): This is a refinement of the Result type from the core crate.
// We should move it to another crate potentially.
//

#![allow(unused)]

use flux_rs::attrs::*;

#[extern_spec]
#[refined_by(valid: bool)]
enum Option<T> {
    #[flux::variant(Option<T>[{valid: false}])]
    None,
    #[flux::variant((T) -> Option<T>[{valid: true}])]
    Some(T),
}

#[extern_spec]
#[refined_by(okay: bool)]
enum Result<T, E> {
    #[flux::variant((T[@okay])->Result<T, E>[{okay: true}])]
    Ok(T),
    #[flux::variant((E[@error]) -> Result<T, E>[{okay: false}])]
    Err(E),
}

