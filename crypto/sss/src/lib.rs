#[macro_use]
extern crate error_chain;

extern crate base64;
extern crate merkle_sigs;
extern crate protobuf; // I don't want to use protobufs; but this could be a good way to learn about them?
extern crate rand;
extern crate ring;

pub mod errors;