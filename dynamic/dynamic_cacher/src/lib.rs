/// TODO:
/// (1) use a HashMap to make this work better
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HashMap: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "this");
        m.insert(1, "is");
        m.insert(2, "a test");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    static ref NUMBER: u32 = times_one_hunna(3);
}

fn times_one_hunna(n: u32) -> u32 { n * 100 }

fn main() {
    println!("The map currently has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("A expensive calculation on a static results in: {}.", *NUMBER);
}

// consider using the above to build a better dynamic cacher
// I'd rather think of something though instead of a generic implementation
// because a generic implementation doesn't make sense to me rn without a real use case
// like I'll think of a use case and then build a relatively generic dynamic_cacher
// but I don't really know what a dynamic cacher means without a use case
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation:T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}