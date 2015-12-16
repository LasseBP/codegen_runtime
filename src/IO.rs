use std::fmt::Debug;

pub fn writeval<T: Debug>(val: T) -> bool {
	print!("{:?}", val);
	true
}

pub fn print<T: Debug>(val: T) {
	print!("{:?}", val);
}

pub fn println<T: Debug>(val: T) {
	println!("{:?}", val);
}