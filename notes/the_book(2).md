# Coding Rust Cheatsheet (2 of 3)

Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.

These are notes from [The Rust Book](https://doc.rust-lang.org/book/), but they also may draw from [Steve Donovan's Gentle Intro](https://stevedonovan.github.io/rust-gentle-intro/readme.html). **These notes only cover the last 7 chapters of The Book** 

* [Closure](#closure)
* [Iterator](#iterator)
    * [Loops vs. Iterators](#loopsvsiterators)
* [Cargo and Crates](#cargo)
    * [Cargo Workspaces](#workspace)
* [Smart Pointers](#smartpointers)
    * [```Box<T>```](#boxt)
    * [```Rc<T>```](#rct)
    * [```Ref<T>```](#reft)
* [Concurrency](#concurrency)
    * [Threads](#threads)
    * [Message Passing](#messagepassing)
    * [Shared State](#sharedstate)
    * [Sync and Send](#syncsend)

## Closures: Anonymous Functions that Capture the Environment <a name="closure"></a>
Rust's closures are anonymous functions that you can store in a variable or pass as arguments to other functions. Closures capture values from  the scope in which they're called, enabling code reuse and behavior customization.

```
let closure_example = |param| {
    println!("This is just an example closure");
    param
}
closure_example(15);
```

Closures are usually short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler is reliably able to infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables.

As with variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary. 

```
let closure_example_annotated = |param: u32| -> u32 {
    println!("Annotated example");
    param
}
```

Closure definitions will have one concrete type inferred for each of their parameters and for their return value. The compiler will not allow calling the same closure twice with two different inferred types (ie ```closure_example("fire")``` and then ```closure_example(32)```).

---
**Storing Closures Using Generic Parameters and the ```Fn``` Traits**<br>

The code pattern known as *memoization* or *lazy evaluation* involves creating a struct to hold the closure as well as the value returned by calling the closure. Specifically, the struct executes the closure only if we need the return value and it will cache the result, thereby enabling the rest of the code to reuse this result.

Every struct definition requires the types for each of its fields. To define structs, enums, or function parameters that use closures, we use generics and trait bounds. All closures implement at least one of the traits: ```Fn```, ```FnMut``` or ```FnOnce```.

We add types to the ```Fn``` trait bound to represent the types of the parameters and return values the closures must have to match this trait bound. 

```
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
```

This ```Cacher``` struct has a ```calculation``` field of the generic type ```T```. The trait bounds on ```T``` specify that it's a closure by using the ```Fn``` trait. 

> While functions can implement all of the ```Fn``` traits, closures are better for caputuring a value from the environment.

The ```value``` field has type ```Option<32>```. Before executing the closure, the ```value``` is ```None```. Once code using ```Cacher``` asks for the *result* of the closure, the ```Cacher``` executes the closure and stores the result in a ```Some``` variant in the ```value``` field. This ensures that if the code asks for the result of the closure again, ```Cacher``` can return the result stored in the ```Some``` variant. Here is the logic in code form:

```
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
```

The ```Cacher::new``` function takes a generic parameter ```T```, which we've defined as maintaining the same trait bound as the ```Cacher``` struct. ```Cacher::new``` returns a ```Cacher``` instance which holds the closure that was specified in the ```calculation``` field as well as ```None``` in the ```value``` field (because we haven't executed the closure yet). 

> The fields for ```Cacher``` are kept private because we want to manage the struct fields' values instead of letting the calling code change these values directly.

Even so, this implementation suffers a fundamental problem whereby if we call the closure with one value, then the value is cached and we can no longer call the closure with a new value. We can modify this implementation to alleviate this problem by holding a hash map instead of a single value. We can also introduce more generic parameters to increase the flexibility of this implementation (thereby allowing it to accept and/or return more than one type). *I plan to explore these improvements in a workout app, which may be pushed to my github soon.*

---
**Tradeoffs for Using Closures vs Functions**<br>
When a closure captures a value from its environment, it uses memory to store the values for use in te closure body. This incurs more memory overhead than a normal function, which does not capture its environment.

Closures can capture values from their environment in three ways, each of which directly map to the ways that a function can take a parameter:
1. **taking ownership** => ```FnOnce``` consumes the variables it captures from its enclosing scope (otherwise known as the closure's *environment*). To consume the captured variables, the closure must take ownership of these variables and move them into the closure once it is defined. 
> The ```Once``` part of the name indicates that the closure can't take ownership of the same variables more than once (so it can be called only once).<br>
2. **borrowing mutably** => ```FnMut``` can change the environment because it mutably borrows values.
3. **borrowing immutably** => ```Fn``` borrows values from the environment immutably.

> When you create a closure, Rust will infer which trait to use based on how the closure manages the variables from the environment. All closures implement ```FnOnce```, but closures that don't move the captured variables also implement ```FnMut```, and closures that don't need mutable access to the captured variables also implement ```Fn```. 

We use the ```move``` keyword before the parameter list to force closures to take ownership of the values that it uses in the environment. Example:

```
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x); 
    // the above line will most definitely will give us a compilation error
}
```

> Easy way to start with closures is to use ```Fn``` and the compiler will tell you whether you need to add ```FnMut``` or ```FnOnce``` based on what happens inside the closure body. 

## Processing a Series of Items with Iterators <a name="iterator">

In Rust, iterators are *lazy* such that they have no effect until we call methods that consume the iterator (to use it). Here's the syntax for creating an iterator over the items in a vector.

```
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

---
**The ```Iterator``` Trait and the ```next``` Method**<br>
All iterators implement the trait ```Iterator```, which has the definition:

```
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

This definition tells us that implementing the ```Iterator``` trait requires defining an ```Item``` type, which is used in the return type of the ```next``` method. 

> The ```Iterator``` trait only requires implementators to define one method: the ```next``` method, which returns one item of the iterator at a time, which is wrapped in ```Some``` and, when the iteration is over, returns ```None```.

Although we don't need to make an iterator mutable for a ```for``` loop (because the loop takes ownership and makes it mutable behind the scenes), we would need to define the iterator as mutable if we are successively calling ```next``` (because each ```next``` call eats up an item from the iterator).

> The values we get from calls to ```next``` are immutable references to the values in the vector. The ```iter``` method produces an iterator over immutable references. If we want to create an iterator that takes ownership of ```v1``` and returns owned values, we can call ```into_iter``` instead of ```iter```. If we want to iterate over mutable references, we can call ```iter_mut``` (instead of ```iter```).

---
**Methods that Consume the Iterator**<br>
The ```Iterator``` trait maintains default methods provided by the standard library, some of which call the ```next``` method in their definition. Methods that call ```next``` are referred to as *consuming adaptors* because calling them uses up the iterator. An example is the ```sum``` method, which takes ownership of the iterator and iterates through items by repeatedly called ```next``` (which therefore consumes the iterator upon each call). As it iterates through the given object (maybe a vector), it adds each item to a running total and returns the total once the iteration is complete.

```
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

> Note that we cannot use ```v1_iter``` after the call to ```sum``` because ```sum``` takes ownership of the iterator that we call.

---
**Methods that Produce Other Iterators**<br>
Other methods defined on the ```Iterator``` trait known as *iterator adaptors* enable you to change iterators into different kinds of iterators. Because all iterators are lazy, you still need to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

```
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

The above code calls the iterator adaptor ```map```, which takes a closure to call on each item to produce a new iterator. but the closure we've specified never gets called because **iterator adaptors are lazy** (so we need to consume the iterator). We can fix this by using the ```collect``` method to consume the iterator and colllect the resulting variables into a collection data type.

```
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

Because ```map``` takes a closure, we can specify any operation that we want to perform on each item. This provides a solid example of how closures enable customized behavior while also reusing the iteration behavior provided by the ```Iterator``` trait.

---
**Using Closures that Capture Their Environment**<br>
The ```filter``` iterator adaptor is a method on an iterator. ```filter``` takes a closure that takes each item from the iterator and returns a Boolean. If the closure returns ```true```, the  value will be included in the iterator produced by ```filter```. Conversely, if the closure returns ```false```, the value cannot be included in the resulting iterator. Here is an example with syntax:

```
#[derive(PartialEq, Debug)]
struct Student {
    name: String,
    score: u32,
}

fn pass_or_fail(students: Vec<Student>, passing_score: u32) -> Vec<Student> {
    students.into_iter()
        .filter(|s| s.score >= passing_score)
        .collect()
}

#[test]
fn filter_by_score() {
    let students = vec![
        Student { name: String::from("Bob"), score: 2300 },
        Student { name: String::from("Alice"), score: 2250 },
        Student { name: String::from("Charlie"), score: 1500 },
    ];

    let passing = pass_or_fail(students, 2000)

    assert_eq!(
        passing,
        vec![
            Student { name: String::from("Bob"), score: 2300 },
            Student { name: String::from("Alice"), score: 2250 },
        ]
    );
}
```

---
**Creating Our Own Iterators with the ```Iterator``` Trait**<br>
If we want to create our own iterators using the ```Iterator``` trait, we only need to provide a definition for the ```next``` method, and we can use all other methods that have default implementations provided by the ```Iterator``` trait. Here's a super simple example:

```
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

Here, we set the associated ```Item``` type for our iterator to ```u32```. This indicates that the iterator will return ```u32``` values. We want our iterator to add 1 to the current state so we initialized ```count``` to 0 so it returns 1 first. If the value of ```count``` is less than 6, ```next``` will return the current value wrapped in ```Some```, but if ```count``` is 6 or higher, our iterator will return ```None```.

Because we already implemented the ```Iterator``` trait by defining the ```next``` method, we now can use any of the ```Iterator``` trait method's default implementations. As an example, let's say that we want to take the values produced by an instance of ```Counter```, pair them with values produced by another ```Counter``` instance after skipping the first value, multiply each pair together, keep only those reulsts that are divisible by 3, and add all the resulting values together, we could do as like this:

```
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                    .map(|(a, b)| a * b)
                                    .filter(|x| x % 3 == 0)
                                    .sum();
    assert_eq!(18, sum);
}
```
> Side Note: consider using [this](https://doc.rust-lang.org/book/2018-edition/ch13-03-improving-our-io-project.html) to improve my zencryption command line tool by incorporating closures and iterators

### Comparing Performance: Loops vs Iterators <a name="loopsvsiterators"></a>

Although iterators are a high-level abstraction, they are compiled down to roughly the same code as if you'd written the lower-level code yourself. Specifically, iterators are one of Rust's *zero-cost abstractions* (<=> the abstraction imposes no additional runtime overhead). Bjarne Stroustrup defined *zero-overhead* in "Foundations of C++"(2012) 
> "In general, C++ implementations obey the zero-overhead principle: What you don't use, you don't pay for. And further: What you do use, you couldn't hand code any better."

When Rust knows the number of iterations for a loop, it can "unroll" the loop. *Unrolling* is an optimization that removes the overhead of the loop controlling code and instead generates repititive code for each iteration of the loop.

## Cargo and Crates <a name="cargo"></a>

> For more details, check out [Cargo Documentation](https://doc.rust-lang.org/cargo/)

*Release profiles* in Rust are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code. Each profile is configured independently of the others!

Cargo has two main profiles:
1. the ```dev``` profile is used implicitly whenever you run ```cargo build```
2. the ```release``` profile requires invoking the ```--release``` flag (```cargo build --release```)

Here are the default values for the ```opt-level``` setting for the ```dev``` and ```release``` profiles in the Cargo.toml file. The ```opt-level``` setting controls the number of optimizations that Rust applies to the code (ranging from 0 to 3). Although applying more optimizations enables faster runtime, it also extends compiling time (so if you're in development mode, it's better to pick a lower ```opt-level```).

```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

---
**Making Useful Documentation Comments**<br>
Documentation comments use three slashes ```///``` instead of two and support Markdown notation for formatting the text. Place documentation comments right before the item that they're documenting. Here's an example (some markdown inception LOL):

```
/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

We can generate the HTML documentation from this documentation comment by running ```cargo doc```. This command runs the ```rustdoc``` tool distributed in Rust and puts the generated HTML documentation in the *target/doc* directory. Running ```cargo doc --open``` builds the HTML and opens the result in a web browser.

Here are a few sections that crate authors commonly use in their documentation:
* **Examples**
* **Panics**: the scenarios in which the function being documented could panic
* **Errors**: if the function returns a ```Result```, describing the kinds of errors that may occur and what conditions might cause those errors to be returned can be helpful to callers so that they can write code to handle the different potential errors.
* **Safety**: If the function is ```unsafe``` to call, there should be a section explaining why the function is unsafe as well as which invariants the function expects the callers to uphold.

> Interestingly, runnning ```cargo test``` will run the code examples in the documentation as tests. This ensures that the examples work with the code.

Comments that appear after ```//!``` provide documentation for the item that contains the comments itself. Typically, these style comments are included at the top of the crate root file (*src/lib.rs*). Documentation comments within items are useful for describing crates and modules especially. Use them to explain the overall purpose of the container to help your users understand the crate's organization.

---
**Exporting a Convenient Public API with ```pub use```**<br>
You can re-export items to make a public structure that's different from the private structure by using ```pub use```. Re-exporting takes a public item in one location and makes it public in another location (as if it were defined in the other location instead).

Utilizing ```pub use``` enables flexibility with respect to structuring the crate internally by allowing you to decouple the internal strcuture from the API presented to users.

To learn how to publish crates to crates.io look [here](https://doc.rust-lang.org/book/2018-edition/ch14-02-publishing-to-crates-io.html).

### Cargo Workspaces <a name="workspace"></a>

A *workspace* is a set of packages that share the same *Cargo.lock* and output directory. An example structure of a workspace could consist of a binary and two libraries such that the binary depends on both libraries.

This section may be more useful in the future and, at that point in time, I can review [this section](https://doc.rust-lang.org/book/2018-edition/ch14-03-cargo-workspaces.html) in [The Book](https://doc.rust-lang.org/book/2018-edition/).

---
**Installing Binaries from Crate.io with ```cargo install```**<br>
All binaries installed with ```cargo install``` are stored in the installation root’s *bin* folder. If you installed Rust using *rustup.rs* and don’t have any custom configurations, this directory will be *$HOME/.cargo/bin*. Ensure that directory is in your ```$PATH``` to be able to run programs you’ve installed with ```cargo install```.

## Smart Pointers <a name="smartpointers"></a>
*Smart Pointers* are data structures that act as pointers while also maintaining additional metadata and capabilities. While references are pointers that only borrow data, smart pointers *own* the data they point to. Examples of smart pointers already covered include ```String``` and ```Vec<T>``` (because they own some memory and allow you to manipulate it). In addition, both of these types have metadata (ie their capacity) as well as extra capabilities and guarantees (ie ```String``` ensures that it will always be valid UTF-8).

Generally, smart pointers are implemented using structs that implement the ```Deref``` and ```Drop``` traits. The ```Deref``` trait enables an instance of the smart pointer to behave as a reference, which allows code that works with either references or smart pointers. On the other hand, ```Drop``` trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope.

Most common smart pointers in the standard library: <br>
* [```Box<T>```](#boxt) for allocating values on the heap 
* [```Rc<T>```](#rct), which is a reference counting type that enables multiple ownership
* [```Ref<T>```](#reft) and ```RefMut<T>``` (accessed through ```RefCell<T>```), a type that enforces the borrowing rules at runtime instead of compile time.

> *interior mutability pattern* such that an immutable type exposes an API for mutating an interior value

> *reference cycles*: how they can leak memory and how to prevent them

### Using ```Box<T>``` to Point to Data on the Heap <a name="boxt"></a>
Boxes (denoted as type ```Box<T>```) allow you to store data on the heap rather than the stack. Even so, the pointer to the heap data is stored on the stack. For this reason, boxes don't have performance overhead (other than storing their data on the heap instead of on the stack). Examples of common use cases:
* When you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size
* When you have a large amount of data and you want to transfer ownership but ensure the data won't be copied
* When you want to own a value and you care only that it is a type that implements a specific trait instead of being a particular type

```
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```
Here we store an ```i32``` value on the heap using a box. The program prints ```b = 5``` (so we can access the data in the box similar to how we would if this data were on the stack). Similar to owned values, when a box goes out of scope, it is deallocated (both for the box stored on the stack and the data it points to on the heap).

---
**Enabling Recursive Types with Boxes**<br>
At compile time, Rust must know how much space a type takes up, but this doesn't work for *recursive types* (ie where a value can have as part of itself another value of the same type). This nesting of values makes it impossible for Rust to effectively allocate space. Even so, boxes have a known size so we can insert a box in a recursive type definition to enable recursive types.

**Cons List**<br>
A *cons list* is a data structure from Lisp. The cons function is used the following context: "to cons *x* onto *y*", which means to construct a new container instance by putting the element *x* at the start of this new container, followed by the container *y*.  Each item in a cons list contains two elements: the value of the current item and the next item. The last item in the list contains only a ```Nil``` value without any next item. 

To demonstrate what Boxes can do, we'll implement a cons list that holds only ```i32``` values. 

```
enum List {
    Cons(i32, List),
    Nil
}

use List::{Const, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```
**This won't compile** because there's no indication of how much space is required to store a ```List``` variant that is recursive (because it holds another value of itself directly). Instead, we can use ```Box<T>``` to get a recursive type with a known size. 

Rust knows how much space is necessary to store a ```Box<T>``` because this is just a pointer to data stored in the heap (and a pointer's size doesn't change based on the data that it's pointing to). To fix our code, we can put a ```Box<T>``` inside our ```Cons``` variant (instead of another ```List``` value directly). The ```Box<T>``` pints to the next ```List``` value that will be on the heap rather than inside the ```Cons``` variant. 

```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3, 
                Box::new(Nil))))));
}
```

This ```Cons``` variant requires the size of an ```i32``` as well as the space required to store the box's pointer data. Because the ```Nil``` variant doesn't store any values, it needs less space than the ```Cons``` variant. This tells us that any ```List``` value will take up the size of an ```i32``` as well as the size of a box's pointer data.

> Boxes only provide the indirection and box allocation (so they don't have any other special capabilities like those of other smart pointer types). Likewise, they don't have the performance overhead incurred by the special capabilities of other smart pointer types. 

When a ```Box<T>``` value goes out of scope, the heap data that the box is pointing to is cleaned up because of the ```Drop``` trait implementation. ```Box<T>``` is treated like a reference because it implements the ```Deref``` trait.

---
**Treating Smart Pointers like Regular References with the ```Deref``` Trait**<br>
We can customize the behavior of the *dereference operator*, ```*```, by implementing the ```Deref``` trait. Specifically, implementing this trait makes it possible for smart pointers to work in a similar way as references. 

```
fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
This is the syntax of using a dereference operator to follow a reference to an ```i32``` value. Without the dereference operator, we would get a compiler error that tells us that we can't compare a reference to an integer.

We can rewrite this code with ```Box<T>``` instead of reference. 

```
fn main() {
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
We can use the dereference operator to follow the box's pointer in the same way that we did when ```y``` was a reference. The only difference with this code is that ```y``` is set to be an instance of a box pointing to the value in ```x``` (instead of a reference pointing to the value of ```x```).

---
**Defining Our Own Smart Pointer**<br>

```
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```
The ```type Target = T;``` syntax defines an associated type for the ```Deref``` trait to use. Associated types are a slightly different way of declaring a generic parameter. The body of the ```deref``` method contains ```&self.0```, which ensures that ```deref``` returns a reference to the value we want to access with the ```*``` operator.

> The ```deref``` method provides the compiler with the ability to take a value of any type that implements ```Deref``` and call the ```deref``` method to get a ```&``` reference that it knows how to dereference.

> When we entered ```*y``` in the above code, Rust actually runs this code behind the scenes: ```*(y.deref())```. Specifically, Rust substitutes the ```*``` operator with a call to the ```deref``` method, and then a plain dereference so that we don't have to consider whether or not we need to call the ```deref``` method. 

---
**Implicit Deref Coercions with Functions and Methods**<br>
*Deref coercion* converts a reference to a type that implements ```Deref``` into a reference to a type that ```Deref``` can convert the original type into. Deref coercion occurs automatically when we pass a reference to a particular type's value as an argument to a function or method that doesn't match the parameter type in the function or method definition. A sequence of calls to the ```deref``` method converts the type we provided into the type required by the parameter.

To see how deref coercion works, we can utilize the ```MyBox<T>``` type that we previously defined. 

```
// -- snip definition of MyBox<T> --

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

Calling ```hello``` with a reference to a ```MyBox<String>``` value works because of deref coercion. Indeed, we are calling the ```hello``` function with ```&m```, a reference to a ```MyBox<String>```. Because we implemented ```Deref``` on ```MyBox<T>```, Rust can convert ```&MyBox<String>``` to ```&String``` by calling ```deref```.  Rust calls ```deref``` again to turn the ```&String``` to ```&str``` (which matches the ```hello``` function's definition).

Here's how the code would work if Rust didn't implement deref coercion...ew!
```
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```
For clarity, ```(*m)``` dereferences the ```MyBox<String>``` into a ```String```. Then the ```&``` and ```[..]``` take a string slice of the ```String``` that is equalt o the whole string to match the signature of ```hello```. 

> When the ```Deref``` trait is defined for the types involved, Rust will analyze the types and use ```Deref::deref``` as many times as necessary to get a reference to match the parameter's type. 
> Because the number of times that ```Deref::deref``` needs to be inserted is resolved at compile time, there is no runtime penalty for taking advantage of deref coercion.

Rust does deref coercion when it finds types and trait implementations in three cases:
* From ```&T``` to ```&U``` when ```T: Deref<Target=U>```
* From ```&mut T``` to ```&mut U``` when ```T: DerefMut<Target=U>```
* From ```&mut T``` to ```&U``` when ```T:Deref<Target=U>```

While Rust will coerce a mutable reference to an immutable one, the reverse is not allowed: immutable references will never coerce to mutable references. This follows from the borrowing rules that mandates that, if you have a mutable reference, it must be the only reference to the data. The Rust compiler can't guarantee that the immutable reference (that is being converted to an immutable reference) is the only immutable reference (which would be necessary because we are instantiating an immutable reference).

---
**Running Code on Cleanup with the ```Drop``` Trait**<br>
Implementing the ```Drop``` trait allows you to customize what happens when a value is about to go out of scope. This trait is almost always used in the context of implementing smart pointers, but it can also be used to release resources like files or network connections. ```Box<T>``` customizes ```Drop``` to deallocate the space on the heap to which the box points.

> In other languages, the programmmer has to call code to free memory or resources every time they finish using an instance of a smart pointer. Conversely, in Rust, you can specify that some code must be run whenever a value goes out of scope and the compiler will insert this code automatically. This ensures against the leaking of resources.

To use the ```Drop``` trait, you must implement one method named ```drop``` that takes a mutable reference to ```self```.  As an example, we'll implement a ```CustomSmartPointer``` struct whose only custom functionality is that it will print ```Dropping CustomSmartPointer!``` when the instance goes out of scope.

```
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
```

Rust automatically called ```drop``` for us when our instances went out of scope (calling the code specified in our implementation of the ```Drop``` trait). Variables are dropped in the reverse order of their creation (ie ```d``` is dropped before ```c```). 

---
**Dropping a Value Early with ```std::mem::drop```**<br>
Sometimes we may want to force the ```drop``` method. An example may be if the smart pointer manages locks and we want to release the lock so other code in the same scope can acquire the lock. In this situation, Rust doesn't allow us to call the ```Drop``` trait's ```drop``` method manually; instead we have to call the ```std::memory::drop``` function provided by the standard libray if we want to force a value to be dropped before the end of its scope.

Rust doesn't let us call ```drop``` explicitly because Rust would still automatically call ```drop``` on the value at the end of ```main```. This would cause a *double free error* (because Rust would be trying to clean up the same value twice). Here's an example of how we could apply ```std::mem::drop``` to explicitly drop a value before it goes out of scope. 

```
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    drop(c);
}
```

### ```Rc<T>```, The Reference Counted Smart Pointer <a name="rct"></a>
To enable multiple ownership, Rust has a type called ```Rc<T>``` (which is an abbreviation for *reference counting*). The ```Rc<T>``` type keeps track of the number of references to a value to determine whether or not the value is still in use. We use ```Rc<T>``` when we want to allocate some data on the heap for multiple parts of our program to read and can't determine at compile time which part will finish using the data last.

> ```Rc<T>``` is only for use in single-threaded scenarios.

Now we're going to use ```Rc<T>``` to share data in the ```Cons``` example (instead of ```Box<T>```, which we used before). Each ```Cons``` will now hold a value and a ```Rc<T>``` pointing to a ```List```. When we create ```b```, instead of taking ownership of ```a```, we will clone the ```Rc<List>``` that ```a``` is holding, thereby increasing the number of references from one to two and allowing ```a``` and ```b``` to share ownership of the data in the given ```Rc<List>```.  In addition, we will clone ```a``` when creating ```c```, thereby increasing the number of references from two to three. 

Every time we call ```Rc::clone```, the reference count to the data within the ```Rc<List>``` will increase, and the data won't be cleaned up unless there are zero references to it.

```
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&b));
}
```
Because ```Rc<T>``` isn't in the prelude, we add a ```use``` statement to bring ```Rc<T>``` into scope. 

> Note that we could have called ```a.clone()``` rather than ```Rc::clone(&a)```, but Rust's convention is to use ```Rc::clone``` here. The implementation of ```Rc::clone``` doesn't make a deep copy of all the data like most types' implementations of ```clone```. Instead, the call to ```Rc::clone``` only increments the reference count, which doesn't take much time (deep copies take a lot of time). 

```
-- snip from stuff above main in code right above --
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons, (10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

This code prints the following, which shows us how the reference count changes. 

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

> Using ```Rc<T>``` allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of the owners exist. 

By utilizing immutable references, ```Rc<T>``` allows you to share data between multiple parts of your program for reading only. If ```Rc<T>``` allowed you to have multiple references too, it would risk violating one of the borrowing rules. We can use an interior mutability pattern and the ```RefCell<T>``` type that you can use in conjunction with an ```Rc<T>``` to work with this immutability restriction. We'll discuss this next!

### Reference Cycles with ```Ref<T>``` <a name="reft"></a>
Rust doesn't allow data races at compile time => memory leaks are memory safe in Rust. Even so, **Rust allows memory leaks** by using ```Rc<T>``` and ```RefCell<T>``` -- it is possible to create references where items refer to each other in a cycle. This creates memory leaks because the reference count of each item in the cycle will never reach 0 (and, therefore, the values will never be dropped).

**Creating a Reference Cycle**<br>
```
# fn main() {}
use std::rc::Rc;
use std::cell:RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}
```

This cons list definition holds a ```RefCell<T>```, which thereby allows us to modify the object to which a ```Cons``` variant is referring. The ```tail``` method makes it convenient for us to access the second item if we have a ```Cons``` variant. We can use the code above to create a reference cycle of two ```List``` valyes pointing to each other.

```
# use List::{Cons, Nil};
# use std::rc::Rc;
# use std::cell::RefCell;
# #[derive(Debug)]
# enum List {
#     Cons(i32, RefCell<Rc<List>>),
#     Nil,
# }
#
# impl List {
#     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
#         match *self {
#             Cons(_, ref item) => Some(item),
#             Nil => None,
#         }
#     }
# }
#
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // this next line would overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```
First, we created a ```Rc<List>``` instance holding a ```List``` value in the variable ```a``` with an initial list of ```5, Nil```. We then create an ```Rc<List>``` instance holding another ```List``` value in the variable ```b``` that contains the value 10 and points to the list in ```a```. To create a cycle, we modify ```a``` so it points to ```b``` instead of ```Nil```. We do that by using the ```tail``` method to get a reference to the ```RefCell<Rc<List>>``` in ```a```, which we put in the variable ```link```. Then, we use the ```borrow_mut``` method on the ```RefCell<Rc<List>>``` to change the value inside from an ```Rc<List>``` that holds a ```Nil``` value to the ```Rc<List>``` in ```b```.

The output of running this code is:

```
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2
b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
b rc count after changing a = 2
a rc count after changing a = 2
```
> If we had uncommented the last ```println!```, Rust would've tried to print this cycle with ```a``` pointing to ```b``` pointing to ```a``` and so forth until it overflows the stack.

---
**Preventing Reference Cycles: Turning an ```Rc<T>``` into a ```Weak<T>```**<br>
> So far, we've demonstrated how ```Rc::clone``` increases the ```strong_count``` of an ```Rc<T>``` instance, and an ```Rc<T>``` instance is only cleaned up if its ```strong_count``` is 0. 

We can also create a *weak reference* to the value within an ```Rc<T>``` instance by calling ```Rc::downgrade``` and passing a reference to the ```Rc<T>```. When we call ```Rc::downgrade```, we get a smart pointer of type ```Weak<T>```. Rather than increasing the ```strong_count``` in the ```Rc<T>``` instance by 1, calling ```Rc::downgrade``` increases the ```weak_count``` by 1. 

> Similar to ```strong_count```, ```weak_count``` is used by the ```Rc<T>``` type to keep track of how many ```Weak<T>``` references exist. Even so, the ```weak_count``` doesn't need to be 0 for the ```Rc<T>``` instance to be cleaned up.

> Likewise, strong references are how you can share ownership of an ```Rc<T>``` instance. Conversely, weak references don't express an ownership relationship. They don't cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.

Note that the value that ```Weak<T>``` references might have been dropped. Therefore, to do anything with the value that a ```Weak<T>``` is pointing to, you must make sure that the value still exists. This is achieved by calling the ```upgrade``` method on a ```Weak<T>``` instance (which returns an ```Option<Rc<T>>```). If the ```Rc<T>``` value hasn't been dropped yet, the ```Option<Rc<T>>``` will take the form of ```Some```; if the ```Rc<T>``` value had been dropped, this returns a ```None```. Because ```upgrade``` returns an ```Option<T>```, Rust will ensure that the ```Some``` case and the ```None``` cases are handled (and therefore there will not be an invalid pointer).

---
**Creating a Tree Data Structure: a ```Node``` with Child Nodes**<br>
For this example. we'll create a struct named ```Node``` that holds its own ```i32``` values as well as references to its children ```Node``` values:
```
use std::rc::Rc;
use std::cell:RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```
We are trying to build a ```Node``` that owns its children. Therefore, we want to share that ownership with variables sowe can access each ```Node``` in the tree directly. To do this, we define the ```Vec<T>``` items to be values of type ```Rc<Node>```. We also want to modify which nodes are children of another node. For this purpose, we use ```RefCell<T>``` in ```children``` around the ```Vec<Rc<Node>>>```.

Now, we can use our struct definition to create ```Node``` instance named ```leaf``` with the value 3 and no children, and another instance named ```branch``` with the value 5 and ```leaf``` as one of its children.

```
# use std::rc::Rc;
# use std::cell:RefCell;
#
# #[derive(Debug)]
# struct node {
#   value: i32,
#   children: RefCell<Vec<Rc<Node>>>,
# }
#
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```
This code creates a ```leaf``` node with no children and a ```branch``` node with ```leaf``` as one of its children.

We clone the ```Rc<Node>``` in ```leaf``` and store that in ```branch```, meaning the ```Node``` in ```leaf``` now has two owners: ```leaf``` and ```branch```. We can get from ```branch``` to ```leaf``` through ```branch.children```, but there's no way to get from ```lead``` to ```branch```. The reason is that ```leaf``` has no reference to ```branch``` and doesn't know they're related. We want ```leaf``` to know that ```branch``` is its parent. 

**Adding a Reference from a Child to its Parent**<br>
To make the child node aware of its parent, we need to add a ```parent``` field to our ```Node``` struct definition. We know that the ```parent``` field cannot contain an ```Rc<T>``` because that would create a reference cycle with ```leaf.parent``` pointing to ```branch``` and ```branch.children``` pointing to ```leaf``` (which would cause the ```strong_count``` values to never be 0).

> Consider it is in terms of the basic relationships between ```parent``` and ```child```. If a parent node is drooped, its child nodes should be dropped as well. However, a child should not own its parent: if we drop a child node, the parent should still exist.

Instead of ```Rc<T>```, we'll make the type of ```parent``` use ```Weak<T>```, specifically a ```RefCell<Weak<Node>>```. Therefore, our ```Node``` struct definition looks like this:
```
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```
A node will be able to refer to its parent node but doesn't own its parent. 

With this in mind, let's update main:
```
# use std::rc::{Rc, Weak};
# use std::cell::RefCell;
#
# #[derive(Debug)]
# struct Node {
#     value: i32,
#     parent: RefCell<Weak<Node>>,
#     children: RefCell<Vec<Rc<Node>>>,
# }
#
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

This demonstates how we can create a ```leaf``` node with a weak reference to its parent node ```branch```.

> Creating the ```leaf``` node resembles how we created the ```leaf``` node before except for the ```parent``` field; ```leaf``` starts out without a parent, so we create a new empty ```Weak<Node>``` reference instance. 

When we try to get a reference to the parent of ```leaf``` by using the ```upgrade``` method, we get a ```None``` value; this is shown by the output from the first ```println!``` statement:
```
leaf parent = None
```

When we create the ```branch``` node, it will also have a new ```Weak<Node>``` reference in the ```parent``` field. Even so, we declare ```leaf``` as one of the children of ```branch```. Once we have the ```Node``` instance in ```branch```, we can modify ```leaf``` to give it a ```Weak<Node>``` reference to its parent. We use the ```borrow_mut``` method on the ```RefCell<Weak<Node>>``` in the ```parent``` field of ```leaf```, and then we use the ```Rc::downgrade``` function to create a ```Weak<Node>``` reference to ```branch``` from the ```Rc<Node>``` in ```branch```.

This means that when we print the parent of ```leaf``` again, we'll get a ```Some``` variant holding ```branch```: now ```leaf``` can access its parent! When we print ```leaf```, we also avoid the cycle that eventually ended in a stack overflow...the ```Weak<Node>``` references are printed as ```(Weak)```:

```
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

---
**Visualizing Changes to ```strong_count``` and ```weak_count```**<br>
We'll wrap this up by showing how the ```strong_count``` and ```weak_count``` values of the ```Rc<Node>``` instances change by creating a new inner scope and moving the creation of ```branch``` into that scope. By doing so, we can see what happens when ```branch`` is created and then dropped when it goes out of scope.

```
# use std::rc::{Rc, Weak};
# use std::cell::RefCell;
#
# #[derive(Debug)]
# struct Node {
#     value: i32,
#     parent: RefCell<Weak<Node>>,
#     children: RefCell<Vec<Rc<Node>>>,
# }
#
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch stron= {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

> This creates ```branch``` in an inner scope while examining strong and weak reference counts

After ```leaf``` is created, its ```Rc<Node>``` has a strong count of 1 and a weak count of 0. In the inner scope, we create ```branch``` and associate it with ```leaf```, at which point when we print the counts, the ```Rc<Node>``` in ```branch``` will have a strong count of 1 and a weak count of 1 (for ```leaf.parent``` pointing to ```branch``` with a ```Weak<Node>```). When we print the counts in ```leaf```, we'll see it will have a strong count of 2, because ```branch``` now has a clone of the ```Rc<Node>``` of ```leaf``` stored in ```branch.children```, but will still have a weak count of 0.

When the inner scope ends, ```branch``` goes out of scope and the strong count of the ```Rc<Node>``` decreases to 0, so its ```Node``` is dropped. The weak count of 1 from ```leaf.parent``` has no bearing on whether or not ```Node``` is dropped (so we don't have any memory leak).

If we try to access the parent of ```leaf``` after the end of the scope, we'll get ```None``` again. At the end of the program, the ```Rc<Node>``` in ```leaf``` has a strong count of 1 and a weak count of 0 (because the variable ```leaf``` is now the only reference to the ```Rc<Node>``` again).

**Summary!**<br>
* The ```Box<T>``` type has a known size and points to data allocated on the heap
* The ```Rc<T>``` type keeps track of the number of references to data on the heap so that data can have multiple owners
* The ```RefCell<T>``` type (via its interior mutability) gives us a type that we can use when we need an immutable type but need to change an inner value of that type; it also enforces the borrowing rules at runtime instead of compile time.

## Concurrency <a name="concurrency"></a>
*Concurrent programming* is when different parts of the program execute independently. *Parallel programming* is where different parts of the program execute at the same time. As computers take advantage of multiple processors, Rust strives to enable **fearless concurrency**.

* [Threads](#threads)
* [Message Passing](#messagepassing)
* [Shared State](#sharedstate)
* [Sync and Send](#syncsend)

### Using Threads to Run Code Simultaneously <a name="threads></a>
Splitting computation into multiple threads can improve performance, but it also adds complexity. Because threads can run simultaneously, there's no inherent guarantee about the order in which threads execute. This can lead to the follwoing problems
* **Race conditions**: threads are accessing data or resources in an inconsistent order
* **Deadlocks**: two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing

Many operating systems provide an API for creating new threads. The model where a language calls the operating system APIs to create threads is referred to as *1:1*, indicating one operating system thread per one language thread. Conversely, some programming languages use *green threads*, which execute in the context of a different number of OS threads. The green-threaded model is called the *M:N* model: there are ```M``` green threads per ```N``` OS threads. 

For Rust, the runtime support is considered to be significant. *Runtime* in this context refers to the code that is included by the language in every binary. The code can be large or small depending on the language, but every non-assembly language has some amount of runtime code. Smaller runtimes have fewer features but have the advantage of resulting in smaller binaries. Although some languages are willing to increase runtime size in exchange for more features, Rust aims to have nearly no runtime; this ensures that it does not compromise on being able to call into C to maintain performance.

> The green-threading M:N model requires a larger language runtime to maintain threads. With this in mind, the Rust standard library only provides an implementation of 1:1 threading. There are crates that implement M:N threading (but these inherently trade overhead for aspects such as more control over which threads run when and lower costs of context switching).

---
**Creating a New Thread with ```spawn```**<br>
To create a new thread, we call the ```thread::spawn``` function and pass it a closure containing the code we want to run in the new thread.

```
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
The output of this program looks like, but it might be slightly different every time! This is because it depends on how the OS schedules the threads. 
```
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```
The new thread is stopped when the main thread ends!

**Waiting for All Threads to Finish Using ```join``` Handles**<br>
We can fix the problem of the spawned thread not getting to run completely by saving the return value of ```thread::spawn``` in a variable. The return type of ```thread::spawn``` is ```JoinHandle```. A ```JoinHandle``` is an owned value that, when we call the ```join``` method on it, will wait for its thread to finish. Here's how to use this:

```
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```
Calling ```join``` on the handle blocks the thread currently running until the thread represented by the handle terminates. *Blocking* a thread means that the thread is prevented from performing work or exiting. Here's the output:
```
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```
The two threads continue alternating, but the main thread waits because of the call to ```handle.join()``` and does not end until the spawned thread is finished. If we had instead placed ```handle.join().unwrap()``` before the ```for``` loop in ```main```, then the main thread would wait for the spawned thread to finish before running its ```for``` loop. This illustrates the significance of where ```join``` is called.

**Using ```move``` Closures with Threads**<br>
When used alongside ```thread::spawn```, the ```move``` closure allows you to use data from one thread in another thread.

> Remember that we can use the ```move``` keyword before the parameter list of a closure to force the closure to take ownership of the values it uses in the environment. 

This technique is very useful when creating new threads in order to transfer ownership of values from one thread to another. Specifically, to use data from the main thread in the spawned thread, the spawned thread's closure must capture the values it needs. By adding the ```move``` keyword before the closure, we force the closure to take ownnership of values it's using rather than allowing Rust to infer that it should borrow the values (if we don't do this, we'll get a compiler error).

```
use std::thread;'

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```
By telling Rust to move ownership of ```v``` to the spawned thread, we're guaranteeing Rust that that the main thread won't use ```v``` anymore. The ```move``` keyword overrides Rust's conservative default of borrowing; it doesn't let us violate the ownership rules.

### Using Message Passing to Transfer Data Between Threads <a name="messagepassing"></a>
An increasingly popular approach to ensure safe concurrency is *message passing*, where threads or actors communicate by sending each other messages containing data.

> Slogan from Golang documentation: "Do not communicate by sharing memory; instead, share memory by communicating."

The major tool utilized by Rust for accomplishing message-sending concurrency is the *channel* (which is implemented by Rust's standard library). A channel has two halves: a transmitter and a receiver. One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. A channel is said to be *closed* if either the transmitter or receiver half is dropped.

Here's the syntax for creating a channel, but it won't compile because Rust can't tell which type of values we want to send over the channel.

```
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

We create a new channel using the ```mpsc::channel``` function. ```mpsc``` stands for *multiple producer, single consumer*. The way Rust's standard library implements channels means a channel can have multiple sending ends that produce values but only one *receiving* end that consumes those values. 

```mpsc:channel``` function returns a tuple, the first element of which is the sending end and the second element is the receiving end. The abbreviatons ```tx``` and ```rx``` are traditionally used in many fields for *transmitter* and *receiver* respectively. We're using a ```let``` statement with a pattern that destructures the tuples. 

Next, we'll move the transmitting end into a spawned thread and have it send one string so the spawned thread is communicating with the main thread. We also get the value from the receiving end of the channel in the main thread.

```
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```
We're using the ```thread::spawn``` to create a new thread and then using ```move``` to move ```tx``` into the closure so the spawned thread owns ```tx```. The spawned thread needs to own the transmitting end of the channel to be able to send messages through the channel.

The transmitting end has a ```send``` method that takes the value we want to send. The ```send``` method returns a ```Result<T, E>```, so if the receiving end has already been dropped and there's nowhere to send a value, the send operation will return an error.

> We use ```unwrap``` here to panic in case of an error, but we're hip to better error handling strategies.

The receiving end of a channel has two useful methods: ```recv``` and ```try_recv```. We're using ```recv```, short for receive, which will block the main's thread execution and wait until a value is sent down the channel. Once a value is sent, ```recv``` will return it in a ```Result<T, E>```. When the sending end of the channel closes, ```recv``` returns an error to signal that no more values will be coming.

The ```try_recv``` method doesn't block, but will instead return a ```Result<T, E>``` immediately: an ```Ok``` value holding a message if one is available and an ```Err``` value if there aren't any messages this time. Using ```try_recv``` is useful if this thread has other work to do while waiting for messages: we could write a loop that calls ```try_recv``` every so often, handles a message if one is available, and otherwise does other work for a little while until checking again.

This is our code's output:

```
Got: hi
```

**Channels and Ownership Transference**<br>
> It is important to note that the transmitting end's ```send``` function takes ownership of its parameter, and, when the value is moved, the received takes ownership of it. This prevents us from accidentally using the value again after sending it!

Now we'll write some code so that the spawned thread sends multiple messages and pauses for a second between each message. 

```
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("don't"),
            String::from("have"),
            String::from("haters"),
            String::from("only"),
            String::from("fans"),
            String::from("in"),
            String::from("denial"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```
The spawned thread has a vector of strings that we want to send to the main thread. We iterate over them, sending each on individually, and pause between each.

In the main thread, we're not calling the ```recv``` function explicitly. Instead, we're treating ```rx``` as an iterator such that for each value received, we print it. When the channel closes, the iteration ends. Our output is exactly what you'd expect.

**Creating Multiple Producers by Cloning the Transmitter**<br>
We'll create multiple threads that all send values to the same receiver. We can do so y cloning the transmitting half of the channel.

```
// --snip--
let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("don't"),
        String::from("have"),
        String::from("haters"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("only"),
        String::from("fans"),
        String::from("in"),
        String::from("denial");
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}

// --snip--
```
This shows how we can send multiple messages from multiple producers. Before we create the first spawned thread, we call ```clone``` on the sending end of the channel. This enables us to pass a new sending handle to the first spawned thread. We pass the original sending end of the channel to a second spawned thread. This code pattern provides two threads, each sending different messages to the receiving end of the channel.

### Shared-State Concurrency <a name="sharedstate"></a>
> Another slogan from the Go language documentation: "communicate by sharing memory"

Channels are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time. 

**Using Mutexes to Allow Access to Data from One Thread at a Time**<br>
*Mutex* is an abbreviation for *mutual exclusion*; a mutex allows only one thread to accesss some data at a given time. To access the data in a mutex, a thread must first sigmal that it wants access by asking to acquire the mutex's *lock*. The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data. 

> the mutex is described as *guarding* the data it holds via the locking system

To use mutexes, you have to remember two rules:
* you must attempt to acquire the lock before using the data
* when you're done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock

Because of Rust's type system and ownership rules, you can't get locking and unlocking wrong.

**The API of ```Mutex<T>```**
In a single threaded context, here's the syntax for using a mutex
```
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```
We create a ```Mutex<T>``` using the associated function ```new```. To access data in the mutex, we use the ```lock``` method to acquire the lock. This lock will block the current thread so it can't do any work until it's our turn to have the lock.

> The call to ```lock``` would fail if another thread holding the lock panicked. In that case, no one would ever be able to get the lock, so we've chosen to ```unwrap``` and have this thread panic if we're in that situation.

After we've acquired the lock, we can treat the return value (named ```num```) as a mutable reference to the data inside. The type system ensures that we acquire a lock before using the value in ```m```: ```Mutex<i32>``` is not an ```i32```, so we must acquire the lock to be able to use the ```i32``` value. Indeed, the type system won't let us access the inner ```i32``` otherwise.

> ```Mutex<T>``` is a smart pointer. More accurately, the call to ```lock``` returns a smart pointer called ```MutexGuard```. This smart pointer implements ```Deref``` to point at our inner data; the smart pointer also has a ```Drop``` implementation that releases the lock automatically when a ```MutexGuard``` goes out of scope (which occurs at the end of the inner scope). 

After the lock is dropped, we print the mutex value and see that we were able to change the inner ```i32``` to 6. Output:
```
m  = 6
```

**Multiple Ownership with Multiple Threads**<br>
> ```Rc<T>``` is not safe to share across threads. When ```Rc<T>``` manages the reference count, it adds to the count for each call to ```clone``` and subtract from the count when each clone is dropped. But it does not use concurrency primitives to ensure that changes to the count can't be interrupted by another thread. 

Instead, we need a type exactly like ```Rc<T>``` that makes changes to the reference count in a thread-safe way!

**Atomic Reference Counting with ```Arc<T>```**<br>
```Arc<T>``` is a type like ```Rc<T>``` that is safe to use in concurrent situations. The *a* stands for *atomic*, meaning it's an *atomically reference counted* type. Atomics work like primitive types but are safe to share across threads.

> thread safety comes with a performance penalty that you only want to pay when you really need to. If you're just performing operations on values within a single thread, your code can run faster if it does not have to enforce the guarantees atomic provide.

Here's some syntax as an example:

```
use std::synx::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&clone);
        let handle = spread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
We use ```Arc<T>``` to wrap the ```Mutex<T>``` to be able to share ownership across multiple threads.

The output is: 
```
Result: 10
```

> ```Mutex<T>``` provides interior mutability similar to the ```Cell``` family. In the same way that we use ```RefCell<T>``` to mutate contents inside an ```Rc<T>```, we use ```Mutex<T>``` to mutate contents inside an ```Arc<T>```.

Note that ```Mutex<T>``` comes with the risk of creating *deadlocks*. These occur when an operation needs to lock two resources and two threads have acquired one of the locks, causing them to wait for each other forever (similar to how ```Rc<T>``` can create reference cycles). 

### Extensible Concurrency with the ```Sync``` and ```Send``` Traits <a name = "syncsend"></a>
Two concurrency concepts are embedded in the language: the ```std::marker``` traits ```Sync``` and ```Send```.

The ```Send``` marker trait indicates that ownership of the type implementing ```Send``` can be transferred between threads. Almost every Rust type is ```Send```, but there are some exceptions, including ```Rc<T>``` (it can't be ```Send``` because if you cloned an ```Rc<T>``` and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time). Likewise, Rust's type system and trait bounds ensure that you can never accidentally send an ```Rc<T>``` value across threads unsafely.

> Any type composed entirely of ```Send``` types is automatically marked as ```Send```. Almost all primitive types are ```Send```, except raw pointers which we'll discuss later

The ```Sync``` marker trait indicates that it is safe for the type implementing ```Sync``` to be referenced from multiple threads. Any type ```T``` is ```Sync``` as long as ```&T``` is ```Send``` (ie the reference can be safely sent to another thread). Primitive types are ```Sync``` and types composed entirely of types that are ```Sync``` are also ```Sync```.

> ```Rc<T>``` is also not ```Sync``` (for the same reason, it's not ```Send```). The ```RefCell<T>``` type and the family of related ```Cell<T>``` types are not ```Sync``` because the implementation of borrow checking that ```RefCell<T>``` does at runtime isn't thread-safe.

> The smart pointer ```Mutex<T>``` is ```Sync``` and can be used to share access with multiple threads.

**Implementing ```Send``` and ```Sync``` Manually is Unsafe** (check out The Rustonomicon for more details)