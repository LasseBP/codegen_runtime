use Set;
use Seq;
use std::hash::Hash;
use std::fmt::Debug;

pub fn set2seq<T: Hash + Eq + Clone>(set: Set<T>) -> Seq<T> {
	set.iter().cloned().collect()
}

pub fn val2seq_of_char<T: Debug>(val: T) -> Seq<char> {
	strseq!(format!("{:?}", val))
}