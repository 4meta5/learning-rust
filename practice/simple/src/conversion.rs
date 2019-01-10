/// Conversion practice
/// [](https://doc.rust-lang.org/std/convert/index.html)
/// [](https://doc.rust-lang.org/rust-by-example/conversion/from_into.html)
/// `From` and `Into` traits are inherently linked
/// This is used often in the Parity Codebase

/// The `From` trait allows for a type to define how to create itself from another type
/// The `Into` trait is the reciprocal of the `From` trait

/// use std::convert::AsMut [doc](https://doc.rust-lang.org/std/convert/trait.AsMut.html)
/// A cheap, mutable reference-to-mutabel reference conversion
/// AsMut auto-derefernces if the inner type is a mutable reference
/// Example: Box<T> implements AsMut<T>
fn add_one<T: AsMut<u64>>)(num: &mut T) {
    *num.as_mut() += 1;
}

let mut boxed_num = Box::new(0);
add_one(&mut boxed_num);
assert_eq!(*boxed_num, 1);

/// use std::convert::AsRef [doc](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
/// A cheap reference-to-reference converrsion - used to convert a value to a reference value within generic code
/// AsRef auto-dereferences if the inner type isa  reference or a mutable reference
/// Example: Both `String` and `&str` implement `AsRef<str>`
fn is_hello<T: AsRef<str>>(s: T) {
    assert_eq!("hello", s.as_ref());
}

let s = "hello";
is_hello(s);

let s = "hello".to_string();
is_hello(s);

/// use std::convert::From [doc](https://doc.rust-lang.org/std/convert/trait.From.html)
/// Simple and safe type conversion in to `Self`
/// An example usage for error handling
use std::io::{self, Read};
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let mut file = std::fs::File::open("test")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}
/// use std::convert::Into [doc](https://doc.rust-lang.org/std/convert/trait.Into.html)
/// A conversion that consume `self` -- the reciprocal of `From`
/// Example of `String` implementing `Into<Vec<u8>>
fn is_hello2<T: Into<Vec<u8>>>(s: T) {
   let bytes = b"hello".to_vec();
   assert_eq!(bytes, s.into());
}

let s = "hello".to_string();
is_hello2(s);

/// in substrate/core/sr-primitives/trait.rs:

/// Auxiliary wrapper that holds an api instance and binds it to the given lifetime
use core;
pub struct ApiRef<'a, T>(T, core::marker::PhantomData<&'a ()>);

impl<'a, T> From<T> for ApiRef<'a, T> {
    fn from(api: T) -> Self {
        ApiRef(api, Default::default())
    }
}

/// Extensible conversion trait. Generic over both source and destination types.
pub trait Convert<A, B> {
    /// Make conversion
    fn convert(a: A) -> B;
}

/// Simple trait similar to `Into`, except that it can be used to convert numberics between
/// each other
pub trait As<T> {
    /// Convert forward (aka `Into::into`)
    fn as_(self) -> T;
    /// Convert backward (aka `From::from`)
    fn sa(T) -> Self;
}