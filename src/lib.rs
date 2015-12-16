#![allow(non_snake_case, non_upper_case_globals)]

extern crate rand;

#[macro_use] 
extern crate lazy_static;

#[macro_use] 
extern crate itertools;

mod token;
#[macro_use]
mod macros;
mod seq;
mod set;
mod map;
mod util;
mod f64_wrapper;

//std libs
pub mod IO;
pub mod MATH;
pub mod VDMUtil;

pub use self::token::Token;
pub use self::seq::Seq;
pub use self::set::Set;
pub use self::map::Map;
pub use self::f64_wrapper::F64;