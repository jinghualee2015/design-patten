use crate::visitor::Visitor;

#[allow(unused)]
mod visitor;

/// A struct of two integer values.
///
/// It is going to be an output of `Visitor` trait which is defined for the type
/// in `visitor.rs`
#[derive(Default, Debug)]
pub struct TwoValueStruct {
    a: i32,
    b: i32,
}


/// A struct of values array.
///
/// It is going to be an output of `Visitor` trait which is defined for the type
/// in `visitor.rs`
#[derive(Default, Debug)]
pub struct TwoValueArray {
    ab: [i32; 2],
}

/// `Deserializer` trait defines methods that can parse either a string or
/// a vector, it accepts a visitor which knows how to construct a new object
/// of a desired type (in our case, `TowValueArray` and `TwoValueStruct`).
trait Deserializer<V: Visitor> {
    fn create(visitor: V) -> Self;
    fn parse_str(&self, input: &str) -> Result<V::value, &'static str> {
        Err("parse_str is unimplemented")
    }
    fn parse_vec(&self, input: Vec<i32>) -> Result<V::value, &'static str> {
        Err("parse_vec is unimplemented")
    }
}

struct StringDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for StringDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_str(&self, input: &str) -> Result<V::value, &'static str> {
        let input_vec = input
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Ok(self.visitor.visit_vec(input_vec))
    }
}

struct VecDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for VecDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_vec(&self, input: Vec<i32>) -> Result<V::value, &'static str> {
        Ok(self.visitor.visit_vec(input))
    }
}


fn main() {
    let deserializer = StringDeserializer::create(TwoValueStruct::default());
    let result = deserializer.parse_str("123 456");
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValueStruct::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValueArray::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    println!(
        "Error: {}",
        deserializer.parse_str("123 456").err().unwrap()
    )
}
