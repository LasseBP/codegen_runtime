use std::collections::HashSet;
use std::collections::hash_set::{Iter, IntoIter};
use std::hash::{Hash, Hasher};
use std::fmt;
use std::ops::{Deref, DerefMut, BitOr, BitAnd, Sub};
use std::iter::{Iterator, FromIterator, IntoIterator};
use util::*;
use Map;
use Seq;

#[derive(Clone, PartialEq, Eq)]
pub struct Set<T: Hash + Eq> {
	inner: HashSet<T>
}

impl<T: Hash + Eq + Clone> Set<T> {
	
	pub fn new() -> Set<T> {
		Set { inner: HashSet::new() }
	}
	
	pub fn in_set(&self, value: T) -> bool {
		self.inner.contains(&value)
	}
	
	pub fn card(&self) -> u64 {
		self.inner.len() as u64
	}
	
	pub fn union(&self, other: Set<T>) -> Set<T> {
		self | &other
	}
	
	pub fn inter(&self, other: Set<T>) -> Set<T> {
		self & &other
	}
	
	pub fn difference(&self, other: Set<T>) -> Set<T> {
		self - &other
	}
	
	pub fn is_subset(&self, other: Set<T>) -> bool {
		self.inner.is_subset(&other)
	}
	
	pub fn is_psubset(&self, other: Set<T>) -> bool {
		self.len() < other.len() && self.is_subset(other)
	}
	
	pub fn powersets(&self) -> Set<Set<T>> {
		let mut iter = self.iter().cloned();
		
		if let Some(first_elem) = iter.next() {
			let rest: Set<T> = iter.collect();
			
			let powersets = rest.powersets();
			
			let mut sets = Set::new();
			for	set in &powersets {
				let mut new_set = set.clone();
				new_set.insert(first_elem.clone());
				
				sets.insert(new_set);
				sets.insert(set.clone());
			}
			sets		
			
		} else {
			let mut sets = Set::new();
			sets.insert(Set::new());
			sets
		}
	}
	
	pub fn exists1<P>(&self, mut pred: P) -> bool where
        P: FnMut(T) -> bool
    {
        let filtered = self.iter().filter(|&e| pred(e.clone()));
        filtered.count() == 1
    }
    
    pub fn iota<P>(&self, mut pred: P) -> T where
        P: FnMut(T) -> bool
    {
        let mut filtered = self.iter().filter(|&e| pred(e.clone()));
        let item = filtered.next().expect("Iota does not select a result.");
        
        // make sure only one item satisfied the expression.
        match filtered.next() {
        	None => item.clone(),
        	Some(_) => panic!("Iota selects more than one result."),
        }
    }
    
    pub fn exists<P>(&self, mut pred: P) -> bool where
        P: FnMut(T) -> bool
    {
        self.iter().any(|e| pred(e.clone()))
    }
    
    pub fn forall<P>(&self, mut pred: P) -> bool where
        P: FnMut(T) -> bool
    {
        self.iter().all(|e| pred(e.clone()))
    }
    
    pub fn be_such_that<P>(&self, mut pred: P) -> T where
        P: FnMut(T) -> bool
    {
        self.iter().find(|&e| pred(e.clone()))
        		   .expect("Let Be St found no applicable bindings")
        		   .clone()
    }
    
    fn compr<P, E, O, B>(&self, mut pred: P, mut expr: E) -> B where
        P: FnMut(T) -> bool,
        E: FnMut(T) -> O,
        O: Eq + Hash + Clone,
        B: FromIterator<O>
    {
    	let f = |e: &T| {
    		if pred(e.clone()) {
    			Some(expr(e.clone()))
    		} else {
    			None
    		}
    	}; 
    	
        self.iter().filter_map(f).collect()
    }
    
    pub fn set_compr<P, E, O>(&self, pred: P, expr: E) -> Set<O> where
        P: FnMut(T) -> bool,
        E: FnMut(T) -> O,
        O: Eq + Hash + Clone
    {
        self.compr(pred, expr)
    }
       	  
    pub fn map_compr<P, E, V, K>(&self, pred: P, expr: E) -> Map<K,V> where
        P: FnMut(T) -> bool,
        E: FnMut(T) -> (K,V),
        K: Eq + Hash + Clone,
        V: Eq + Hash + Clone
    {
        self.compr(pred, expr)
    }
}

impl<T: Hash + Eq + Clone + Ord> Set<T> {
	
	pub fn seq_compr<P, E, O>(&self, mut pred: P, mut expr: E) -> Seq<O> where
        P: FnMut(T) -> bool,
        E: FnMut(T) -> O,
        O: Eq + Hash + Clone
    {
    	let f = |e: &T| -> Option<O> {
    		if pred(e.clone()) {
    			Some(expr(e.clone()))
    		} else {
    			None
    		}
    	};
    	
        let mut v: Vec<_> = self.iter().cloned().collect();
        v.sort();
        v.iter().filter_map(f).collect()
    }
}

impl<T: Hash + Eq + Clone> Set<Set<T>> {
	
	pub fn dunion(&self) -> Set<T> {
		self.iter().flat_map(|s| s).cloned().collect()
	}
	
	pub fn dinter(&self) -> Set<T> {		
		
		if let Some(s) = self.iter().next() {
			self.iter().fold(s.clone(),|int, s| s.intersection(&int).cloned().collect())
		} else {
			Set::new()
		}
	}
}

impl Set<i64> {
	pub fn range(start: f64, end: f64) -> Set<i64> {
		let start = start.ceil() as isize;
		let end = (end.floor() as isize) + 1;
		
		(start..end).map(|i| i as i64).collect()
	}
}

impl<T: Hash + Eq> Default for Set<T> {
    fn default() -> Set<T> {
    	Set{ inner: Default::default() }
    }
}

impl<'a, 'b, T> Sub<&'b Set<T>> for &'a Set<T>
    where T: Eq + Hash + Clone
{
    type Output = Set<T>;

    /// Returns the difference of `self` and `rhs` as a new `Set<T>`.
    fn sub(self, rhs: &Set<T>) -> Set<T> {
        self.inner.difference(&rhs.inner).cloned().collect()
    }
}

impl<'a, 'b, T> BitAnd<&'b Set<T>> for &'a Set<T>
    where T: Eq + Hash + Clone
{
    type Output = Set<T>;

    /// Returns the intersection of `self` and `rhs` as a new `Set<T>`.
    fn bitand(self, rhs: &Set<T>) -> Set<T> {
        self.inner.intersection(&rhs.inner).cloned().collect()
    }
}

impl<'a, 'b, T> BitOr<&'b Set<T>> for &'a Set<T>
    where T: Eq + Hash + Clone
{
    type Output = Set<T>;

    /// Returns the union of `self` and `rhs` as a new `Set<T>`.
    fn bitor(self, rhs: &Set<T>) -> Set<T> {
        self.inner.union(&rhs.inner).cloned().collect()
    }
}

impl<T: Hash + Eq + Clone> Hash for Set<T> {
	
	/// https://github.com/rust-lang/rust/issues/21182
    fn hash<H>(&self, state: &mut H) where H: Hasher {
    	let set_hash = self.into_iter().fold(0, |sum, val| sum ^ get_hash(val));
    	set_hash.hash(state);
    }
}

impl<T> FromIterator<T> for Set<T>
    where T: Eq + Hash + Clone
{
    fn from_iter<I: IntoIterator<Item=T>>(iterable: I) -> Set<T> {
        Set { inner: iterable.into_iter().collect() }
    }
}

impl<'a, T> IntoIterator for &'a Set<T>
    where T: Eq + Hash + Clone
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T> IntoIterator for Set<T>
    where T: Eq + Hash + Clone
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        self.inner.into_iter()
    }
}

impl<T: Hash + Eq + Clone> Deref for Set<T> {
	type Target = HashSet<T>;
	
    fn deref(&self) -> &HashSet<T> {
        &self.inner
    }
}

impl<T: Hash + Eq + Clone> DerefMut for Set<T> {
    fn deref_mut(& mut self) -> & mut HashSet<T> {
        &mut self.inner
    }
}

impl <T: Hash + Eq + Clone> Into<HashSet<T>> for Set<T> {
    fn into(self) -> HashSet<T> {
        self.inner
    }
}

impl <T: Hash + Eq + Clone> From<HashSet<T>> for Set<T> {
    fn from(s: HashSet<T>) -> Set<T> {
        Set { inner: s}
    }
}

impl<T: Hash + Eq + Clone + fmt::Display> fmt::Display for Set<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let item_string = self.iter().map(|i| format!("{}",i))
								   .collect::<Vec<_>>()
								   .join(", ");
		
        write!(f, "{{{}}}", item_string)
    }
}

impl<T: Hash + Eq + Clone + fmt::Debug> fmt::Debug for Set<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Seq;
    use Map;

    #[test]
    fn equality() {
    	let s1 = set!{1,2,3,3};
    	let s2 = set!{2,3,1};
    	
        assert_eq!(s1, s2);
    }
    
    #[test]
    fn inequality() {
    	let s1 = set!{1,2,3};
    	let s2 = set!{1,2};
    	
        assert!(s1 != s2);
    } 
    
    #[test]
    fn in_set() {
    	let s1 = set!{1,2,3};
    	
        assert!(s1.in_set(1));
    } 
    
    #[test]
    fn not_in_set() {
    	let s1 = set!{1,2,3};
    	
        assert!(!s1.in_set(4));
    }
    
    #[test]
    fn range() {    	
        assert_eq!(set!{2,3,4,5}, Set::range(1.2f64, 5.7f64));
    }
    
    #[test]
    fn union() {
    	let s1 = set!{1,2,3};
    	let s2 = set!{4,5};
    	
        assert_eq!(set!{1,2,3,4,5}, s1.union(s2));
    }
    
    #[test]
    fn inter() {
    	let s1 = set!{1,2,3,4};
    	let s2 = set!{4,5};
    	
        assert_eq!(set!{4}, s1.inter(s2));
    }
    
    #[test]
    fn difference() {
    	let s1 = set!{1,2,3,4};
    	let s2 = set!{4,5};
    	
        assert_eq!(set!{1,2,3}, s1.difference(s2));
    }
    
    #[test]
    fn is_subset() {
    	let s1 = set!{1,2,3,4,5};
    	let s2 = set!{4,5};
    	let s3 = set!{4,5};
    	
        assert!(s2.is_subset(s1.clone()));
        assert!(s2.is_subset(s3));
        assert!(!s1.is_subset(s2));
    }
    
    #[test]
    fn is_psubset() {
    	let s1 = set!{1,2,3,4,5};
    	let s2 = set!{4,5};
    	let s3 = set!{4,5};
    	
        assert!(s2.is_psubset(s1.clone()));
        assert!(!s2.is_psubset(s3));
        assert!(!s1.is_psubset(s2));
    }
    
    #[test]
    fn dunion() {
    	let ss = set!{set!{1,2}, set!{2,3,4}, set!{2,5,6}};
    	
        assert_eq!(set!{1,2,3,4,5,6}, ss.dunion());
    }
    
    #[test]
    fn dinter() {
    	let ss = set!{set!{1,2}, set!{2,3,4}, set!{2,5,6}};
    	
        assert_eq!(set!{2}, ss.dinter());
    }
    
    #[test]
    fn powersets() {
    	let exp_ps = set!{set!{1,2}, set!{1}, set!{2}, set!{}};
    	
        assert_eq!(exp_ps, set!{1,2}.powersets());
    }
    
    #[test]
    fn display_formatting() {
    	let s1_string = set!{1}.to_string();
 	    	
        assert_eq!("{1}",s1_string);
    }
    
    #[test]
    fn iota_1() {
    	let s1 = set!{1,2,3};
 	    	
        assert_eq!(1 ,s1.iota(|i| i == 1));
    }
    
    #[test]
    #[should_panic(expected = "Iota does not select a result.")]
    fn iota_2() {
    	let s1 = set!{1,2,3};
    	let _ = s1.iota(|i| i == 4);
    }
    
    #[test]
    #[should_panic(expected = "Iota selects more than one result.")]
    fn iota_3() {
    	let s1 = set!{1,2,3};
    	let _ = s1.iota(|_| true);
    }
    
    #[test]
    fn exists1_1() {
    	let s1 = set!{1,2,3};
 	    	
        assert!(s1.exists1(|i| i == 1));
    }
    
    #[test]
    fn exists1_2() {
    	let s1 = set!{1,2,3};
    	
        assert!(!s1.exists1(|_i| true));
    }
    
    #[test]
    fn exists1_3() {
    	let s1 = set!{strseq!("foo"), strseq!("bar")};
 	    let foo = strseq!("foo");
        assert!(s1.exists1(|i| i == foo));
    }
    
    #[test]
    fn exists_1() {
    	let s1 = set!{strseq!("foo"), strseq!("bar")};
 	    let foo = strseq!("foo");
        assert!(s1.exists(|i| i == foo));
    }
    
    #[test]
    fn exists_2() {
    	let s1 = set!{strseq!("foo"), strseq!("bar")};
 	    let foo = strseq!("baz");
        assert!(!s1.exists(|i| i == foo));
    }
    
    #[test]
    fn cartesian_1() {
    	let s1 = set!{1,2};
		let s2 = set!{3,4};
		let s3 = set!{5,6};
		
		let cart = cartesian_set!(s1, s2, s3);
 		let result = set!{(1,3,5), (1,3,6), 
 						  (1,4,5), (1,4,6),
 						  (2,3,5), (2,3,6), 
 						  (2,4,5), (2,4,6)};
	 	assert_eq!(result, cart);
    }
    
    #[test]
    fn cartesian_2() {

		let cart = cartesian_set!(set!{1,2}, set!{3,4}, set!{5,6});
 		let result = set!{(1,3,5), (1,3,6), 
 						  (1,4,5), (1,4,6),
 						  (2,3,5), (2,3,6), 
 						  (2,4,5), (2,4,6)};
	 	assert_eq!(result, cart);
    }
       
    #[test]
    fn forall1() {
    	let s1 = set!{strseq!("foo"), strseq!("bar")};
 	
        assert!(s1.forall(|i| i.len() == 3));
    }
    
    #[test]
    fn forall2() {
    	let s1 = set!{strseq!("foo"), strseq!("bar")};
    	let s2 = set!{strseq!("set"), strseq!("baz")};
    	
    	let result = s1.forall(|i| s2.forall(
    				|j| i.len() == j.len()));
 	
        assert!(result);
    }
    
    #[test]
    fn let_be_such_that() {
    	let result = set!{1,2,3}.be_such_that(|i| i % 2 == 0);
    		
        assert_eq!(2, result);
    }
    
    #[test]
    fn set_comprehension_1() {
    	let result = set!{1,2,3,0}.set_compr(|i| i % 2 == 0, |i| i * 2);
    		
        assert_eq!(set!{4,0}, result);
    }
    
    #[test]
    fn set_comprehension_2() {
    	let s1 = set!{1,2};
		let s2 = set!{3,4};
		let s3 = set!{5,6};
		
		let cart = cartesian_set!(s1, s2, s3);
    	
    	let result = cart.set_compr(|(x,_y,_z)| x == 1, |(_x,y,z)| y*z);
    	
    	let expected = set!{15, 18, 20, 24};
    		
        assert_eq!(expected, result);
    }
    
    #[test]
    fn seq_comprehension() {
    	let result = set!{2,3,1,4}.seq_compr(|i| i % 2 == 0, |i| i * 2);
    		
        assert_eq!(seq![4,8], result);
    }
    
    #[test]
    fn map_comprehension() {
    	let result = set!{2,3,1,4}.map_compr(|i| i % 2 == 0, |i| (i, i * 2 )); 
    		
        assert_eq!(map!{2 => 4, 4 => 8}, result);
    }
}