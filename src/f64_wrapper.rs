use std::hash::{Hash, Hasher};
use std::ops::{ Add, Sub, Mul, Div, Neg };
use std::fmt;

/// Wrapper for `f64`. Is needed to implement `Hash` + `Eq`
#[derive(Copy, Clone, PartialOrd)]
pub struct F64(pub f64);

impl F64 {
	pub fn floor(&self) -> i64 {
		self.0.floor() as i64
	}
	
	pub fn abs(&self) -> F64 {
		F64(self.0.abs())
	}
	
	pub fn pow(&self, other: F64) -> F64 {
		F64(self.0.powf(other.0))
	}
}

impl PartialEq for F64 {
	fn eq(&self, other: &F64) -> bool {
		self.0 == other.0
	}
}

/// Total order only holds for non-NaN values. NaN values are illegal in VDM.
impl Eq for F64 {}

impl Hash for F64 {
    fn hash<S>(&self, state: &mut S) where S: Hasher {
    	use std::mem::transmute;
    	//bit cast (similar to reinterpret_cast in C++) to u64 and perform hash.
        let val = unsafe { transmute::<f64, u64>(self.0) };
        val.hash(state)
    }
}

impl fmt::Display for F64 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {		
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for F64 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {		
        write!(f, "{:?}", self.0)
    }
}

macro_rules! from_impl {
    ($($t:ty)*) => ($(
        
        impl From<$t> for F64 {
            fn from(val: $t) -> F64 {
       	 		F64(val as f64)
    		}
        }
        
        impl From<F64> for $t {
            fn from(val: F64) -> $t {
       	 		val.0 as $t
    		}
        }
    )*)
}

from_impl! { u64 i32 i64 }

impl From<f64> for F64 {
    fn from(val: f64) -> F64 {
        F64(val)
    }
}
            
impl From<F64> for f64 {
    fn from(val: F64) -> f64 {
        val.0
    }
}

impl Add for F64 {
    type Output = F64;

    #[inline]
    fn add(self, other: F64) -> F64 { F64(self.0 + other.0) }
}

impl Sub for F64 {
	type Output = F64;
	
	#[inline]
	fn sub(self, other: F64) -> F64 { F64(self.0 - other.0) }
}

impl Mul for F64 {
	type Output = F64;
	
	#[inline]
	fn mul(self, other: F64) -> F64 { F64(self.0 * other.0) }
}

impl Div for F64 {
	type Output = F64;
	
	#[inline]
	fn div(self, other: F64) -> F64 { F64(self.0 / other.0) }
}

impl Neg for F64 {
	type Output = F64;
	
	#[inline]
	fn neg(self) -> F64 { F64(-self.0) }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn add_1() {
		let f = F64(0.15) + F64(0.15);
		assert_eq!(F64(0.3), f);
	}
	
	#[test]
	fn floor_1() {
		let f = F64(1.23).floor();
		assert_eq!(1, f);
	}
}