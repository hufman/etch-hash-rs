/// Implements the Apache Etch hash algorithm
///
/// Based on https://svn.apache.org/repos/asf/etch/trunk/util/src/main/java/org/apache/etch/util/Hash.java
/// Test cases from https://svn.apache.org/repos/asf/etch/trunk/binding-java/runtime/src/test/java/org/apache/etch/bindings/java/msg/TestIdName.java
/// More test cases from https://etch.apache.org/documentation.html
///
/// ```
/// // Hasher Trait
/// use std::hash::Hasher;
/// use etch_hash::EtchHash;
/// let mut hasher = EtchHash::new();
/// hasher.write("Data to hash".as_bytes());
/// hasher.finish();
/// ```
/// ```
/// // Convenience static methods
/// use etch_hash::hash;
/// hash("Data to hash".as_bytes());
/// ```
///

use std::hash::Hasher;
pub fn hash(data: &[u8]) -> u32 {
    return hash_more(5381, data);
}
pub fn hash_more(start_hash: u32, data: &[u8]) -> u32 {
    let mut result:u32 = start_hash;
    for c in data {
        let bigc = *c as u32;
        let mut big_result:u64 = result.wrapping_shl(16) as u64 + result.wrapping_shl(6) as u64;
        big_result = big_result.wrapping_sub(result as u64) + bigc as u64;
        result = big_result as u32;
    }
    result
}

#[derive(Clone, Debug)]
pub struct EtchHash {
    state: u32,
}
impl EtchHash {
    pub fn new() -> EtchHash {
        EtchHash { state: 5381 }
    }
    pub fn new_with_state(state: u64) -> EtchHash {
        EtchHash { state: state as u32 }
    }
}
impl Hasher for EtchHash {
    fn finish(&self) -> u64 {
        self.state as u64
    }
    fn write(&mut self, bytes: &[u8]) {
        self.state = hash_more(self.state, bytes);
    }
}
