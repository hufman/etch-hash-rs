/// Implements the Apache Etch hash algorithm
///
/// Based on https://svn.apache.org/repos/asf/etch/trunk/util/src/main/java/org/apache/etch/util/Hash.java
/// Test cases from https://svn.apache.org/repos/asf/etch/trunk/binding-java/runtime/src/test/java/org/apache/etch/bindings/java/msg/TestIdName.java
/// More test cases from https://etch.apache.org/documentation.html
///

#[cfg(test)]
mod tests {
    use etch_hash::*;
    use std::hash::Hasher;

    #[test]
    fn null() {
        assert_eq!(5381, hash("".as_bytes()));
    }
    #[test]
    fn single_letter() {
        assert_eq!(0x150a2c9e, hash("c".as_bytes()));
        assert_eq!(352988316, hash("a".as_bytes()));
    }
    #[test]
    fn all_letters() {
        assert_eq!(352988316, hash("a".as_bytes()));
        assert_eq!(1511848646, hash("ab".as_bytes()));
        assert_eq!(669497117, hash("abc".as_bytes()));
        assert_eq!(2300776583, hash("abcd".as_bytes()));
        assert_eq!(3492286878, hash("abcde".as_bytes()));
        assert_eq!(1266308680, hash("abcdef".as_bytes()));
        assert_eq!(3915594783, hash("abcdefg".as_bytes()));
        assert_eq!(2878000137, hash("abcdefgh".as_bytes()));
        assert_eq!(53556896, hash("abcdefghi".as_bytes()));
        assert_eq!(4290539978, hash("abcdefghij".as_bytes()));
    }
    #[test]
    fn long_names() {
        assert_eq!(0x28e34aa1, hash("org.apache.etch.example.binary.binaryExample.f".as_bytes()));
        assert_eq!(0x0972201e, hash("org.apache.etch.example.binary.binaryExample._result_f".as_bytes()));
        assert_eq!(0x28e34a7c, hash("org.apache.etch.example.binary.binaryExample.A".as_bytes()));
    }
    #[test]
    fn long_names_iterative() {
        let result = hash("org.apache.etch.example.binary.binaryExample".as_bytes());
        assert_eq!(0x28e34aa1, hash_more(result, ".f".as_bytes()));
        assert_eq!(0x0972201e, hash_more(result, "._result_f".as_bytes()));
        assert_eq!(0x28e34a7c, hash_more(result, ".A".as_bytes()));
    }

    #[test]
    fn obj_null() {
        let mut hasher = EtchHash::new();
        hasher.write("".as_bytes());
        assert_eq!(5381, hasher.finish());
    }
    #[test]
    fn obj_single_letter() {
        let mut hasher = EtchHash::new();
        hasher.write("c".as_bytes());
        assert_eq!(0x150a2c9e, hasher.finish());
        hasher = EtchHash::new();
        hasher.write("a".as_bytes());
        assert_eq!(352988316, hasher.finish());
    }
    #[test]
    fn obj_long_names_iterative() {
        let mut hasher = EtchHash::new();
        hasher.write("org.apache.etch.example.binary.binaryExample".as_bytes());
        let mut sub_hasher;
        sub_hasher = hasher.clone();
        sub_hasher.write(".f".as_bytes());
        assert_eq!(0x28e34aa1, sub_hasher.finish());
        sub_hasher = hasher.clone();
        sub_hasher.write("._result_f".as_bytes());
        assert_eq!(0x0972201e, sub_hasher.finish());
        sub_hasher = hasher.clone();
        sub_hasher.write(".A".as_bytes());
        assert_eq!(0x28e34a7c, sub_hasher.finish());
        sub_hasher = EtchHash::new_with_state(hasher.finish());
        sub_hasher.write(".A".as_bytes());
        assert_eq!(0x28e34a7c, sub_hasher.finish());
    }
}

pub mod etch_hash {
    /// ```
    /// // Hasher Trait
    /// use std::hash::Hasher;
    /// use etch_hash::etch_hash::EtchHash;
    /// let mut hasher = EtchHash::new();
    /// hasher.write("Data to hash".as_bytes());
    /// hasher.finish();
    /// ```
    /// ```
    /// // Convenience static methods
    /// use etch_hash::etch_hash::hash;
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
}
