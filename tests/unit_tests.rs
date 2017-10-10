extern crate etch_hash;

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
