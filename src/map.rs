use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::hash::{Hash, Hasher};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::iter::{Iterator, FromIterator, IntoIterator};
use util::*;
use Set;

#[derive(Clone, PartialEq, Eq)]
pub struct Map<K: Hash + Eq, V> {
	inner: HashMap<K,V>
}

impl<K, V> Map<K,V> 
	where K: Hash + Eq + Clone,
		  V: Hash + Eq + Clone {
		  	
	pub fn new() -> Map<K,V> {
		Map { inner: HashMap::new() }
	}
	
	pub fn domain(&self) -> Set<K> {
		self.keys().cloned().collect()
	}
	
	pub fn range(&self) -> Set<V> {
		self.values().cloned().collect()
	}
	
	pub fn get(&self, key: K) -> V {
		self.inner.get(&key).expect("No such key in map.").clone()
	}
	
	pub fn get_ref(&mut self, key: K) -> &mut V {
		self.inner.get_mut(&key).expect("No such key in map.")
	}
	
	pub fn merge(&self, other: Map<K,V>) -> Map<K,V> {
		if !self.is_compatible(&other) {
			panic!("Merging requires maps to be compatible");
		} else {
			self.ovrride(other)
		}
	}
	
	pub fn ovrride(&self, other: Map<K,V>) -> Map<K,V> {
		let i1 = self.iter();
		let i2 = other.iter();
		i1.chain(i2).map(|(k,v)| (k.clone(), v.clone())).collect()
	}
	
	pub fn dom_restrict_to(&self, allowed: Set<K>) -> Map<K,V> {
		self.iter().filter(|&(k,_)| allowed.contains(k))
					.map(|(k,v)| (k.clone(), v.clone())).collect()
	}
	
	pub fn dom_restrict_by(&self, restricted: Set<K>) -> Map<K,V> {
		self.iter().filter(|&(k,_)| !restricted.contains(k))
					.map(|(k,v)| (k.clone(), v.clone())).collect()
	}
	
	pub fn rng_restrict_to(&self, allowed: Set<V>) -> Map<K,V> {
		self.iter().filter(|&(_,v)| allowed.contains(v))
					.map(|(k,v)| (k.clone(), v.clone())).collect()
	}
	
	pub fn rng_restrict_by(&self, restricted: Set<V>) -> Map<K,V> {
		self.iter().filter(|&(_,v)| !restricted.contains(v))
					.map(|(k,v)| (k.clone(), v.clone())).collect()
	}
	
	pub fn compose<A: Hash + Eq + Clone>(&self, m2: Map<A,K>) -> Map<A,V> {
		if !m2.range().is_subset(self.domain()) {
			panic!("Range is not a subset of the domain.");
		}
		
		m2.iter().map(|(a,b)| (a.clone(), self[b].clone())).collect()
	}
	
	pub fn inverse(&self) -> Map<V,K> {
		let dom = self.domain();
		let rng = self.range();
		
		if dom.len() != rng.len() {
			panic!("Map must be 1-to-1 to inverse.");
		} else {
			self.iter().map(|(k,v)| (v.clone(),k.clone())).collect()
		}		
	}
	
	fn is_compatible(&self, other: &Map<K,V>) -> bool {
		!self.iter().any(|(k,v)| match other.inner.get(k) {
				None => false,
				Some(other_v) => other_v != v,
			})
	}
}
		  
impl<A: Hash + Eq + Clone> Map<A,A> {

	pub fn iterate(&self, n: u64) -> Map<A,A>
	{
		if n == 0 {
			self.iter().map(|(k,_)| (k.clone(),k.clone())).collect()
		} else if n == 1 {
			self.clone()
		} else if self.range().is_subset(self.domain()) {
			let mut result = self.compose(self.clone());
			
			for i in 2..n {
				println!("{}",i);
				result = result.compose(self.clone());
			}
			
			result
		} else {
			panic!("Range is not a subset of the domain.");
		}
	}

}
	  
		  
impl<K, V> Set<Map<K,V>> 
	where K: Hash + Eq + Clone,
		  V: Hash + Eq + Clone {
	
	pub fn merge(&self) -> Map<K,V> {
		let mut result = Map::new();
		
		for map in self {
			if !result.is_compatible(map) {
				panic!("Merging requires maps to be compatible");
			} else {
				let cloned_iter = map.iter().map(|(k,v)| (k.clone(), v.clone()));
				result.extend(cloned_iter);
			}
		}		
		result
	}
}

impl<K,V> Hash for Map<K,V> 
	where K: Eq + Hash + Clone,
    	  V: Eq + Hash + Clone {
	
	/// https://github.com/rust-lang/rust/issues/21182
    fn hash<H>(&self, state: &mut H) where H: Hasher {
    	let set_hash = self.into_iter().fold(0, |sum, (k,v)| 
    		{ sum ^ get_hash(k) ^ get_hash(v)});
    	set_hash.hash(state);
    }
}	  
    	  
impl<K,V> Default for Map<K,V> 
	where K: Eq + Hash {
    fn default() -> Map<K,V> {
    	Map{ inner: Default::default() }
    }
}

impl<K,V> FromIterator<(K,V)> for Map<K,V>
    where K: Eq + Hash + Clone,
    	  V: Eq + Hash + Clone
{
    fn from_iter<I: IntoIterator<Item=(K,V)>>(iterable: I) -> Map<K,V> {
        Map { inner: iterable.into_iter().collect() }
    }
}

impl<'a, K, V> IntoIterator for &'a Map<K, V>
    where K: Eq + Hash
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

impl<K: Hash + Eq, V> Deref for Map<K,V> {
	type Target = HashMap<K,V>;
	
    fn deref(&self) -> &HashMap<K,V> {
        &self.inner
    }
}

impl<K: Hash + Eq, V> DerefMut for Map<K,V> {
    fn deref_mut(& mut self) -> & mut HashMap<K,V> {
        &mut self.inner
    }
}

impl <K: Hash + Eq, V> Into<HashMap<K,V>> for Map<K,V> {
    fn into(self) -> HashMap<K,V> {
        self.inner
    }
}

impl<K: Hash + Eq + fmt::Display, V: fmt::Display> fmt::Display for Map<K,V> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let item_string = self.iter().map(|(k,v)| format!("{} |-> {}",k,v))
								   .collect::<Vec<_>>()
								   .join(", ");
		
        write!(f, "{{{}}}", item_string)
    }
}

impl<K: Hash + Eq + fmt::Debug, V: fmt::Debug> fmt::Debug for Map<K,V> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {		
        let item_string = self.iter().map(|(k,v)| format!("{:?} |-> {:?}",k,v))
								   .collect::<Vec<_>>()
								   .join(", ");
		
        write!(f, "{{{:?}}}", item_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Set;

    #[test]
    fn equality() {
    	let m1 = map!{1 => "foo", 2 => "bar"};
		let m2 = map!{2 => "bar", 1 => "foo"};
    	
        assert_eq!(m1, m2);
    }
    
    #[test]
    fn inequality() {
    	let m1 = map!{1 => "foo", 2 => "bar"};
		let m2 = map!{2 => "baz", 1 => "foo"};
    	
        assert!(m1 != m2);
    }
    
    #[test]
    fn apply_read() {
    	let m1 = map!{1 => "foo", 2 => "bar"};
    	let e2 = m1.get(2);
    	
        assert_eq!("bar",e2);
    }
    
    #[test]
    fn apply_write() {
    	let mut m1 = map!{1 => "foo", 2 => "bar"};
    	m1.insert(2, "baz");
    	
    	let e2 = m1[&2].clone();
    	
        assert_eq!("baz",e2);
    }
    
    #[test]
    fn domain() {
    	let m1 = map!{1 => "foo", 2 => "bar"};
    	
        assert_eq!(set!{1, 2}, m1.domain());
    }
    
    #[test]
    fn range() {
    	let m1 = map!{1 => "foo", 2 => "bar"};
    	
        assert_eq!(set!{"foo", "bar"}, m1.range());
    }
    
    #[test]
    fn merge() {
    	let m1 = map!{1 => "foo"};
		let m2 = map!{2 => "bar"};
		
		let exp_result = map!{1 => "foo", 2 => "bar"};
    	
        assert_eq!(exp_result, m1.merge(m2));
    }
    
    #[test]
    fn dmerge() {
    	let m1 = map!{1 => "foo"};
		let m2 = map!{2 => "bar"};
		let maps = set!{m1, m2};
		
		let exp_result = map!{1 => "foo", 2 => "bar"};
    	
        assert_eq!(exp_result, maps.merge());
    }
    
    #[test]
    #[should_panic(expected = "Merging requires maps to be compatible")]
    fn merge_incompatible() {
    	let m1 = map!{1 => "foo"};
		let m2 = map!{1 => "bar"};
		
		let _ = m1.merge(m2);
		assert!(true);
    }
    
    #[test]
    fn ovrride() {
    	let m1 = map!{1 => "foo", 2 => "bar"};
    	let m2 = map!{2 => "baz"};
    	
		let exp_result = map!{1 => "foo", 2 => "baz"};
    	
        assert_eq!(exp_result, m1.ovrride(m2));
    }
    
    #[test]
    fn dom_restrict_to() {
    	let m = map!{1 => "foo", 2 => "bar"};
    	let s = set!{2};
    	
        assert_eq!(map!{2 => "bar"}, m.dom_restrict_to(s));
    }
    
    #[test]
    fn dom_restrict_by() {
    	let m = map!{1 => "foo", 2 => "bar"};
    	let s = set!{2};
    	
        assert_eq!(map!{1 => "foo"}, m.dom_restrict_by(s));
    }
    
    #[test]
    fn rng_restrict_to() {
    	let m = map!{1 => "foo", 2 => "bar"};
    	let s = set!{"foo"};
    	
        assert_eq!(map!{1 => "foo"}, m.rng_restrict_to(s));
    }
    
    #[test]
    fn rng_restrict_by() {
    	let m = map!{1 => "foo", 2 => "bar"};
    	let s = set!{"foo"};
    	
        assert_eq!(map!{2 => "bar"}, m.rng_restrict_by(s));
    }
    
    #[test]
    fn composition() {
    	let m1 = map!{"foo" => "bar", "bar" => "kek"};
    	let m2 = map!{1 => "foo", 2 => "bar"};
    	
		let exp_result = map!{1 => "bar", 2 => "kek"};
    	
        assert_eq!(exp_result, m1.compose(m2));
    }
    
    #[test]
    #[should_panic(expected = "Range is not a subset of the domain.")]
    fn composition_panic() {
    	let m1 = map!{"foo" => "bar", "bar" => "kek"};
    	let m2 = map!{1 => "foo", 2 => "bar", 3 => "w00t"};
    	
		m1.compose(m2);
    }
    
    #[test]
    fn iteration0() {
    	let m = map!{ 1 => 2, 2 => 3, 3 => 4, 4 => 1 };
    	
		let i0 = m.iterate(0);
		
		let exp_res0 = map!{ 1 => 1, 2 => 2, 3 => 3, 4 => 4 };
    	
        assert_eq!(exp_res0, i0);
    }
    
    #[test]
    fn iteration1() {
    	let m = map!{ 1 => 2, 2 => 3, 3 => 4, 4 => 1 };
    	
		let i1 = m.iterate(1);
    	
        assert_eq!(m, i1);
    }
    
    #[test]
    fn iteration10() {
    	let m = map!{ 1 => 2, 2 => 3, 3 => 4, 4 => 1 };
    	
		let i10 = m.iterate(10);
		
		let exp_res10 = map!{ 1 => 3, 2 => 4, 3 => 1, 4 => 2 };
    	
        assert_eq!(exp_res10, i10);
    }
    
    #[test]
    fn inverse() {
    	let m = map!{1 => "foo", 2 => "bar"};
    	
		let exp_result = map!{"foo" => 1, "bar" => 2};
    	
        assert_eq!(exp_result, m.inverse());
    }
    
	#[test]
	#[should_panic(expected = "Map must be 1-to-1 to inverse.")]
    fn inverse_panic() {
    	let m = map!{1 => "foo", 2 => "foo"};
    	
		m.inverse();
    }
    
    #[test]
    fn display_formatting() {
 	    let m1 = map!{1 => "foo"};
 	    let m1_string = m1.to_string();
 	    
        assert_eq!("{1 |-> foo}", m1_string);
    }    
     
}