use crate::{TwoValueArray, TwoValueStruct};

/// Visitor can visit one type, do conversions, and output another type
///
/// It's not like all visitors must return a new type, it's just an example
/// that demonstrates the technique.
pub trait Visitor {
    type value;
    fn visit_vec(&self, v: Vec<i32>) -> Self::value;
}

/// Visitor implementation for a struct of two values
impl Visitor for TwoValueStruct {
    type value = TwoValueStruct;

    fn visit_vec(&self, v: Vec<i32>) -> Self::value {
        TwoValueStruct { a: v[0], b: v[1] }
    }
}

/// Visitor implementation for a struct of values array.
impl Visitor for TwoValueArray {
    type value = TwoValueArray;

    fn visit_vec(&self, v: Vec<i32>) -> Self::value {
        let mut ab = [0i32; 2];
        ab[0] = v[0];
        ab[1] = v[1];
        TwoValueArray { ab }
    }
}