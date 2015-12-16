use std::sync::Mutex;
use std::cell::RefCell;
use rand::{Rng, SeedableRng, StdRng};

use F64;

use std::f64::consts::PI as math_pi;

pub const pi: F64 = F64(math_pi);

pub fn sin(v: F64) -> F64 { F64(v.0.sin()) }

pub fn cos(v: F64) -> F64 { F64(v.0.cos()) }

pub fn cot(v: F64) -> F64 { F64(1.0/v.0.tan()) }

pub fn asin(v: F64) -> F64 { F64(v.0.asin()) }

pub fn atan(v: F64) -> F64 { F64(v.0.atan()) }

pub fn acot(v: F64) -> F64 { F64((1.0/v.0).atan()) }

pub fn sqrt(v: F64) -> F64 { F64(v.0.sqrt()) }

pub fn pi_f() -> F64 { pi }

lazy_static! {
	static ref RND: Mutex<RefCell<Option<StdRng>>> = Mutex::new(RefCell::new(None));
}

pub fn srand(seed: i64) {
	srand2(seed);		
}

pub fn rand(top: i64) -> i64 {
	if top == 0 {
		return 0
	} 
	
	let refCell = match RND.lock() {
		Ok(refCell) => refCell,
		Err(poisoned) => poisoned.into_inner(),
	};
	let rnd = refCell.borrow_mut();
	
	if rnd.is_some() {
		rnd.unwrap().gen::<i64>().abs() % top 
	} else {
		top
	}	
}

pub fn srand2(seed: i64) -> i64 {	
	let seedSlice: &[_] = &[seed as usize];
	
	let refCell = match RND.lock() {
		Ok(refCell) => refCell,
		Err(poisoned) => poisoned.into_inner(),
	};
	let mut rnd = refCell.borrow_mut();
	
	if rnd.is_some() {
		rnd.unwrap().reseed(seedSlice);
	} else {
		*rnd = Some(SeedableRng::from_seed(seedSlice));
	}
	
	seed
}

pub fn exp(v: F64) -> F64 { F64(v.0.exp()) }

pub fn ln(v: F64) -> F64 { F64(v.0.ln()) }

pub fn log(v: F64) -> F64 { F64(v.0.log10()) }

pub fn fac(v: u64) -> u64 { 
	assert!(v < 21);
	
	if v == 0 || v == 1 {
		1
	} else {
		v * fac(v-1)
	}
}