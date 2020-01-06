/// Category Theory For Programmers
///
/// (1) the identity function
/// (2) the composition function (take two functions and returns their composition)
/// [src](https://stackoverflow.com/a/45792463)
use std::assert_eq;

// for an arbitrary number of compositions (2+)
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose2( $head, compose!($($tail), +))
    };
}

// (2)
fn compose2<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C 
    where
        F: Fn(A) -> B,
        G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    let add = |x| x + 1;
    let multiply = |x| x * 2;
    let divide = |x| x / 2;
    let id = |x| x;                                 // (1)
    let all_opps = compose!(add, multiply, divide);

    let id_all_ops = compose!(&id, &all_opps);
    let all_opps_id = compose!(&all_opps, &id);
    assert_eq!(id_all_ops(10), all_opps_id(10));
    assert_eq!(all_opps(10), 11);
    println!("pass");
}