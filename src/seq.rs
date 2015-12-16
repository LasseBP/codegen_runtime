use std::{cmp, fmt};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::iter::{Iterator, FromIterator, IntoIterator};
use std::slice::Iter;
use Set;
use Map;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Seq<T: Hash> {
	inner: Vec<T>
}

impl<T: Clone + Hash> Seq<T> {
		
	pub fn new() -> Seq<T> {
		Seq { inner: Vec::new() }
	}
	
	pub fn len(&self) -> u64 {
		self.inner.len() as u64
	}
	
	pub fn get_ref(&mut self, index: u64) -> &mut T {
		&mut self.inner[(index - 1) as usize]
	}
	
	pub fn get(&self, index: u64) -> T {
		self.inner[(index - 1) as usize].clone()
	}
	
	pub fn put(&mut self, index: u64, value: T) {
		self.inner[(index - 1) as usize] = value;
	}
	
	pub fn head(&self) -> T {
		self.get(1)
	}
	
	pub fn tail(&self) -> Seq<T> {
		Seq { inner: self[1..].to_vec() } 
	}
	
	pub fn sub_seq(&self, start: u64, end: u64) -> Seq<T> {
		if start > end || start < 1 {
			return Seq::new();
		}
		
		let start = start - 1;		
		let end = cmp::min(end, self.len());
		
		Seq { inner: self[start as usize..end as usize].to_vec() }
	}
	
	pub fn elems(&self) -> Set<T>
		where T : Eq + Hash + Clone {			
		self.iter().cloned().collect()		
	}
	
	pub fn inds(&self) -> Set<u64> {
		let max_index = self.len() + 1;
		(1..max_index).collect()
	}
	
	pub fn reverse(&self) -> Seq<T> {
		let mut rev = self.clone();
		rev.inner.reverse();
		rev
	}
	
	pub fn conc(&self, right: Seq<T>) -> Seq<T> {
		self.iter().chain(right.iter()).cloned().collect()
	}
	
	pub fn modify(&self, map: Map<usize,T>) -> Seq<T> {
		let mut result = self.clone();
		
		for (idx, value) in &map {
			result[*idx - 1] = value.clone();
		}
		
		result
	}
}

impl<T: Clone + Hash> Seq<Seq<T>> {
	pub fn dconc(&self) -> Seq<T> {
		self.iter().flat_map(|s| s).cloned().collect()
	}
}

impl<T: Hash> Default for Seq<T> {
    fn default() -> Seq<T> {
    	Seq{ inner: Default::default() }
    }
}

impl<T: Hash> FromIterator<T> for Seq<T>
{
    fn from_iter<I: IntoIterator<Item=T>>(iterable: I) -> Seq<T> {
        Seq { inner: iterable.into_iter().collect() }
    }
}

impl<'a, T: Hash> IntoIterator for &'a Seq<T>
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}


impl<T: Hash> Deref for Seq<T> {
	type Target = Vec<T>;
	
    fn deref(&self) -> &Vec<T> {
        &self.inner
    }
}

impl<T: Hash> DerefMut for Seq<T> {
    fn deref_mut(& mut self) -> & mut Vec<T> {
        &mut self.inner
    }
}

impl <T: Hash> Into<Vec<T>> for Seq<T> {
    fn into(self) -> Vec<T> {
        self.inner
    }
}

impl Into<String> for Seq<char> {
    fn into(self) -> String {
        self.iter().cloned().collect()
    }
}

use std::any::Any;

impl<T: Hash + fmt::Display + Any> fmt::Display for Seq<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let is_string = match self.inner.get(0) {
			Some(elem) => Any::is::<char>(elem),
			None => false,
		};
		
		if is_string {
			let item_string = self.inner.iter().map(|i| format!("{}",i))
				   .map(|s| s.trim_matches('\'').to_owned())
				   .collect::<Vec<_>>()
				   .join("");
			write!(f,"{}",item_string)
		} else {
			let item_string = self.inner.iter().map(|i| format!("{}",i))
				   .collect::<Vec<_>>()
				   .join(", ");
        	write!(f, "[{}]", item_string)
		}
    }
}

impl<T: Hash + fmt::Debug + Any> fmt::Debug for Seq<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let is_string = match self.inner.get(0) {
			Some(elem) => Any::is::<char>(elem),
			None => false,
		};
		
		if is_string {
			let item_string = self.inner.iter().map(|i| format!("{:?}",i))
				   .map(|s| s.trim_matches('\'').to_owned())
				   .collect::<Vec<_>>()
				   .join("");
			write!(f,"{}",item_string)
		} else {
        	write!(f, "{:?}", self.inner)
		}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Set;
    use Map;
    
    #[test]
    fn default() {
    	let s: Seq<i32> = Seq::default();
     	
        assert_eq!(seq![], s);
    }

    #[test]
    fn equality() {
    	let s1 = seq![1,2,3];
    	let s2 = seq![1,2,3];
    	
        assert_eq!(s1, s2);
    }
    
    #[test]
    fn str_equality() {
    	let s1 = strseq!["foo"];
    	let s2 = seq!['f','o','o'];
    	
        assert_eq!(s1, s2);
    }
    
    #[test]
    fn inequality() {
    	let s1 = seq![1,2,3];
    	let s2 = seq![1,2];
    	
        assert!(s1 != s2);
    }
    
    #[test]
    fn head() {
    	let s1 = seq![1,2,3];
		let head = s1.head();
		
        assert_eq!(1, head);
    }
    
    #[test]
    fn apply_read_1() {
    	let s1 = seq![1,2,3];
    	let index = 2;
    	
		let elem2 = s1.get(index); 
		
        assert_eq!(2, elem2);
    }
    
    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 99")]
    fn apply_read_2() {
    	let s1 = seq![1,2,3];    	
		let _ = s1.get(100); 
    }
    
    #[test]
    fn apply_write() {
    	let mut s1 = seq![1,2,3];
    	let index = 2u64;
    	
    	s1.put(index, 5);
    	
		let elem2 = s1.get(index);
		
        assert_eq!(5, elem2);
    }
    
    #[test]
    fn tail() {
    	let s1 = seq![1,2,3];
		let tail = s1.tail();
		
        assert_eq!(seq![2,3], tail);
    }
    
    #[test]
    fn elems() {
    	let s1 = seq![1,2,2,3,3];
		let set = s1.elems(); 
		
        assert_eq!(set![1,2,3], set);
    }
    
    #[test]
    fn inds() {
    	let s1 = seq![1,2,2,3,3];
		let inds  = s1.inds();
		
        assert_eq!(set![1,2,3,4,5], inds);
    }
    
    #[test]
    fn reverse() {
    	let s1 = seq![1,2,3];
		let rev  = s1.reverse();
		
        assert_eq!(seq![3,2,1], rev);
    }
    
    #[test]
    fn conc() {
    	let s1 = seq![1,2,3];
    	let s2 = seq![4,5];
		let conc = s1.conc(s2);
		
        assert_eq!(seq![1,2,3,4,5], conc);
    }
    
    #[test]
    fn dconc() {
    	let ss = seq![seq![1,2], seq![3,4], seq![5,6]];    
    	let dconc = ss.dconc();
    	
        assert_eq!(seq![1,2,3,4,5,6], dconc);
    }
    
    #[test]
    fn modify() {
    	let modified = seq![1,2].modify(map!{1 => 5});
    	
        assert_eq!(seq![5,2], modified);
    }
    
    #[test]
    fn sub_seq() {
    	let s1 = seq![1,2,2,3,3];
		let subseq  = s1.sub_seq(2,4);
		
        assert_eq!(seq![2,2,3], subseq);
    }
    
    #[test]
    fn display_formatting() {
    	let s1_string = seq![1,2,3].to_string();
 	    	
        assert_eq!("[1, 2, 3]",s1_string);
    }    
    
    #[test]
    fn display_formatting_chars() {
    	let s1_string = strseq!["foo"].to_string();
 	    
        assert_eq!("foo",s1_string);
    }    
    
    #[test]
    fn strseq_len() {
    	let len = strseq!("🀜").len();
    	
    	assert_eq!(1,len);
    }
}