/// Some notes on combinators with example code
/// following [Herman J. Radtke's Notes](https://hermanradtke.com/2016/09/12/rust-using-and_then-and-map-combinators-on-result-type.html)

/// the `and_then` combinator is a function that 
/// calls a closure if and only if the variant of the `Result` enum type
/// is Ok(T)
let ex1: Result<usize, &'static str> = Ok(5);
let v1 = ex1.and_then(|n: usize| Ok(n * 2));
assert_eq!(Ok(10), v1); // true
let ex2: Result<usize, &'static str> = Err("error");
let v2 = ex2.and_then(|n: usize| Ok(n * 2)); // closure not called
assert_eq!(Err("error"), v2); // true

/// we can chain together multiple `and_then` functions
let ex3: Result<usize, &'static str> = Ok(0);
let v3 = ex3.
    .and_then(|n: usize| {
        if n == 0 {
            Err("cant divide by zero")
        } else {
            Ok(n)
        }
    })
    .and_then(|n: usize| Ok(2 / n)); // closure not called
assert_eq!(Err("cant divide by zero"), v3); // true

// we can use `and_then`
// to flatten the type from `Result<Result<_, _>, _>` to `Result<_, _>`
// by mapping variants in the internal `Result` to the outer `Result`
let ex4: Result<Result<usize, &'static str>, &'static str> = Ok(Ok(5));
let v4 = ex4
    .and_then(|n: Result<usize, &'static str>| {
        n // <--- this is either Ok(usize) or Err(&'static str)
    })
    .and_then(|n: usize| {
        Ok(n*2)
    });
assert_eq!(Ok(10), v4); // true


// we can use a `map` as a replacement for iterating/looping over a list of values
let ex5: Vec<usize> = vec![3];
let v5: Vec<usize> = ex5.iter().map(|n| n * 2).collect();
assert_eq!(vec![6], v5); // true

// when we use `map` with a `Result` type, the `map`
// function calls a closure if and only if the variant of the 
// 'Result` enum is `Ok(t)`.
let ex6: Result<usize, &'static str> = Ok(5);
let v6: Result<usize, &'static str> = ex6.map(|n| n * 3);
assert_eq!(Ok(15), v6); // true
// NOTE: the `map` function always wraps the return value of the closure 
// in the Ok variant

/// Sometimes we are given a `Result` where one or both variants are not the type we want
/// we can use `map` to transform one `Result` type into another
let g1: Result<i32, &'static str> = Ok(5i32);
let d1: Result<usize, &'static str> = g1.map(|n: i32| n as usize);
assert_eq!(Ok(5usize), d1); // true

let v7 = d1.and_then(|n: usize| Ok(n * 2));
assert_eq!(Ok(10), v7); // true

// if the given value is an `Err` variant,
// it is passed through both the `map` and `and_then` functions 
// without the closure being called
let g2: Result<i32, &'static str> = Err("an error");
let d2: Result<usize, &'static str> = g2.map(|n: i32| as usize); // not called
assert_eq!(Err("an error"), d2);

let v8 = d2.and_then(|n: usize| Ok(n * 2)); // not called
assert_eq!(Err("an error"), v8);

/// the `map_err` combinator function is the opposite of `map`
/// because it matches only on `Err(e)` variants of `Result`
enum MyError { Bad };

let g3: Result<i32, &'static str> = Err(MyError::Bad);
let d3: Result<usize, &'static str> = given
    .map(|n: i32| {
        n as usize
    })
    .map_err(|_e: MyError| {
        "bad MyError"
    });
let v9 = d3.and_then(|n: usize| Ok(n * 2));
assert_eq!(Err("bad MyError"), v9);
/// `map` only handles the `Ok(T)` variant of `Result`
/// `map_err` only handles the `Err(E)` variant of `Result`

/// `or_else` function combinator is the opposite of `and_then`
/// it only calls the closure if the result is `Err(E)`