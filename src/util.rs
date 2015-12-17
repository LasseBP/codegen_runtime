use std::hash::{Hash, Hasher, SipHasher};

pub fn get_hash<T: Hash>(val: &T) -> u64 {
    let mut state = SipHasher::new();
    val.hash(&mut state);
    state.finish()
}
