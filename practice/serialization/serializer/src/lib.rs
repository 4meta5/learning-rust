/// Customizable serde serializer
/// [src](substrate/core/serializer)
///
/// Experiment with other implementations later
/// ...more than just using JSON

extern crate serde;
extern crate serde_json;

pub use serde_json::{from_str, from_slice, from_reader, Result, Error};

const PROOF: &str = "quod erat demonstrandum";

/// Serialize the given data structure as a String of JSON
pub fn to_string_pretty<T: serde::Serialize + ?Sized>(value: &T) -> String {
    serde_json::to_string_pretty(value).expect(PROOF)
}

/// Serialize the given data structure as a JSON byte vector
pub fn encode<T: serde::Serialize + ?Sized>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).expect(PROOF)
}

/// Serialize the given data structure as JSON into the IO stream
pub fn to_writer<W: ::std::io::Write, T: serde::Serialize + ?Sized>(writer: W, value: &T) -> Result<()> {
    serde_json::to_writer(writer, value)
}
